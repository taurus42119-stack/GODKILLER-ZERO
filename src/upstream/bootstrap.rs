use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "state", content = "details")]
pub enum EngineStatus {
    Ready {
        model: String,
    },
    StartingService,
    PullingModel {
        model: String,
        percent: f32,
        status_text: String,
    },
    OfflineStandalone {
        reason: String,
    },
}

#[derive(Clone)]
pub struct OllamaBootstrapManager {
    target_model: String,
    current_status: Arc<RwLock<EngineStatus>>,
}

impl OllamaBootstrapManager {
    pub fn new(target_model: impl Into<String>) -> Self {
        Self {
            target_model: target_model.into(),
            current_status: Arc::new(RwLock::new(EngineStatus::StartingService)),
        }
    }

    pub async fn get_status(&self) -> EngineStatus {
        self.current_status.read().await.clone()
    }

    pub async fn set_status(&self, new_status: EngineStatus) {
        let mut status_guard = self.current_status.write().await;
        *status_guard = new_status;
    }

    pub fn target_model_name(&self) -> &str {
        &self.target_model
    }

    pub fn start_background_bootstrap(self: Arc<Self>) {
        tokio::spawn(async move {
            self.execute_bootstrap_lifecycle().await;
        });
    }

    async fn execute_bootstrap_lifecycle(&self) {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(4))
            .build()
            .unwrap_or_default();

        match self.inspect_running_service(&http_client).await {
            Ok(available_models) => {
                self.resolve_model_readiness(&http_client, available_models)
                    .await;
            }
            Err(_) => {
                self.attempt_service_activation(&http_client).await;
            }
        }
    }

    async fn attempt_service_activation(&self, http_client: &reqwest::Client) {
        self.set_status(EngineStatus::StartingService).await;

        if let Some(binary_location) = Self::locate_ollama_executable() {
            if Self::spawn_background_daemon(&binary_location).is_ok() {
                if let Ok(available_models) = Self::poll_service_health(http_client, 20).await {
                    self.resolve_model_readiness(http_client, available_models)
                        .await;
                    return;
                }
            }
        }

        self.set_status(EngineStatus::OfflineStandalone {
            reason:
                "Ollama not detected. Operating in 100% Offline Zero-Dependency Standalone Mode."
                    .into(),
        })
        .await;
    }

    async fn resolve_model_readiness(
        &self,
        http_client: &reqwest::Client,
        available_models: Vec<String>,
    ) {
        let has_model = available_models.iter().any(|model_tag| {
            model_tag.starts_with(&self.target_model) || model_tag.contains(&self.target_model)
        });

        if has_model {
            self.set_status(EngineStatus::Ready {
                model: self.target_model.clone(),
            })
            .await;
            return;
        }

        self.pull_target_model_with_telemetry(http_client).await;
    }

    fn parse_pull_event(line: &str) -> Option<(Option<String>, bool, f32, String)> {
        let event_json = serde_json::from_str::<serde_json::Value>(line).ok()?;
        if let Some(err) = event_json.get("error").and_then(|v| v.as_str()) {
            return Some((Some(err.to_string()), false, 0.0, String::new()));
        }
        let status_label = event_json["status"]
            .as_str()
            .unwrap_or("pulling")
            .to_string();
        let is_success = status_label == "success";
        let completed = event_json["completed"].as_f64().unwrap_or(0.0);
        let total = event_json["total"].as_f64().unwrap_or(1.0);
        let pct = if total > 0.0 {
            ((completed / total) * 100.0) as f32
        } else {
            0.0
        };
        Some((None, is_success, pct.clamp(0.0, 100.0), status_label))
    }

    async fn process_pull_chunk(&self, chunk_str: &str, succeeded: &mut bool) -> Option<String> {
        for line in chunk_str.lines() {
            let Some((err, success, pct, label)) = Self::parse_pull_event(line) else {
                continue;
            };
            if let Some(e) = err {
                return Some(e);
            }
            if success {
                *succeeded = true;
            }
            self.set_status(EngineStatus::PullingModel {
                model: self.target_model.clone(),
                percent: pct,
                status_text: label,
            })
            .await;
        }
        None
    }

    async fn consume_pull_stream(
        &self,
        streaming_response: &mut reqwest::Response,
    ) -> (Option<String>, bool) {
        let mut last_error: Option<String> = None;
        let mut succeeded = false;

        while let Ok(Some(chunk_bytes)) = streaming_response.chunk().await {
            let Ok(chunk_str) = std::str::from_utf8(&chunk_bytes) else {
                continue;
            };
            if let Some(e) = self.process_pull_chunk(chunk_str, &mut succeeded).await {
                last_error = Some(e);
                break;
            }
        }
        (last_error, succeeded)
    }

    async fn pull_target_model_with_telemetry(&self, http_client: &reqwest::Client) {
        self.set_status(EngineStatus::PullingModel {
            model: self.target_model.clone(),
            percent: 0.0,
            status_text: "Initializing model pull stream...".into(),
        })
        .await;

        let pull_endpoint = "http://127.0.0.1:11434/api/pull";
        let request_payload = serde_json::json!({
            "name": self.target_model,
            "stream": true
        });

        let Ok(mut resp) = http_client
            .post(pull_endpoint)
            .json(&request_payload)
            .send()
            .await
        else {
            self.set_status(EngineStatus::OfflineStandalone {
                reason: "Model pull connection failed. Operating in Standalone Mode.".into(),
            })
            .await;
            return;
        };

        if !resp.status().is_success() {
            self.set_status(EngineStatus::OfflineStandalone {
                reason: "Model pull failed: non-success status".into(),
            })
            .await;
            return;
        }

        let (last_err, succeeded) = self.consume_pull_stream(&mut resp).await;
        if last_err.is_none() && succeeded {
            self.set_status(EngineStatus::Ready {
                model: self.target_model.clone(),
            })
            .await;
        } else {
            self.set_status(EngineStatus::OfflineStandalone {
                reason: "Model pull failed or stream closed prematurely without completion confirmation.".into(),
            }).await;
        }
    }

    async fn inspect_running_service(
        &self,
        http_client: &reqwest::Client,
    ) -> Result<Vec<String>, String> {
        let tags_endpoint = "http://127.0.0.1:11434/api/tags";
        let response = http_client
            .get(tags_endpoint)
            .send()
            .await
            .map_err(|e| format!("Failed to reach Ollama endpoint: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Ollama returned non-success status: {}",
                response.status()
            ));
        }

        let json_payload = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Invalid JSON response: {}", e))?;

        let model_collection = json_payload
            .get("models")
            .and_then(|m| m.as_array())
            .map(|models_array| {
                models_array
                    .iter()
                    .filter_map(|model_item| model_item["name"].as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(model_collection)
    }

    fn probe_where_ollama() -> Option<PathBuf> {
        let output = std::process::Command::new("where")
            .arg("ollama")
            .output()
            .ok()?;
        if !output.status.success() {
            return None;
        }
        let paths_str = std::str::from_utf8(&output.stdout).ok()?;
        let first_path = paths_str.lines().next()?;
        let path_candidate = PathBuf::from(first_path.trim());
        if path_candidate.exists() {
            Some(path_candidate)
        } else {
            None
        }
    }

    pub fn locate_ollama_executable() -> Option<PathBuf> {
        let standard_probe_paths = [dirs_hint_localappdata(), dirs_hint_programfiles()];

        for probe_path in standard_probe_paths.into_iter().flatten() {
            if probe_path.exists() {
                return Some(probe_path);
            }
        }

        Self::probe_where_ollama()
    }

    pub fn spawn_background_daemon(binary_location: &Path) -> Result<(), String> {
        let mut process_builder = std::process::Command::new(binary_location);
        process_builder.arg("serve");

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            process_builder.creation_flags(CREATE_NO_WINDOW);
        }

        process_builder
            .spawn()
            .map(|_| ())
            .map_err(|spawn_err| spawn_err.to_string())
    }

    async fn probe_service_tags(http_client: &reqwest::Client) -> Option<Vec<String>> {
        let network_response = http_client
            .get("http://127.0.0.1:11434/api/tags")
            .send()
            .await
            .ok()?;
        if !network_response.status().is_success() {
            return None;
        }
        let json_body = network_response.json::<serde_json::Value>().await.ok()?;
        let model_collection = json_body["models"]
            .as_array()
            .map(|models_array| {
                models_array
                    .iter()
                    .filter_map(|model_item| model_item["name"].as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        Some(model_collection)
    }

    async fn poll_service_health(
        http_client: &reqwest::Client,
        max_attempts: usize,
    ) -> Result<Vec<String>, String> {
        for _ in 0..max_attempts {
            tokio::time::sleep(std::time::Duration::from_millis(600)).await;
            if let Some(models) = Self::probe_service_tags(http_client).await {
                return Ok(models);
            }
        }
        Err("Service failed to become ready within deadline".into())
    }
}

fn dirs_hint_localappdata() -> Option<PathBuf> {
    std::env::var("LOCALAPPDATA").ok().map(|dir_str| {
        PathBuf::from(dir_str)
            .join("Programs")
            .join("Ollama")
            .join("ollama.exe")
    })
}

fn dirs_hint_programfiles() -> Option<PathBuf> {
    std::env::var("ProgramFiles")
        .ok()
        .map(|dir_str| PathBuf::from(dir_str).join("Ollama").join("ollama.exe"))
}
