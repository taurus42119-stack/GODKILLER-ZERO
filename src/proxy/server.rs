use super::handlers::{
    chat_completions_handler, default_workspace_api_handler, engine_bootstrap_api_handler,
    engine_status_api_handler, export_project_config_api_handler, get_startup_api_handler,
    hook_antigravity_api_handler, hook_status_api_handler, inspect_rules_api_handler,
    install_gate_api_handler, interpreter_status_api_handler, list_models_handler, metrics_handler,
    prune_terminal_api_handler, purify_prompt_api_handler, quit_app_api_handler,
    toggle_interpreter_api_handler, toggle_startup_api_handler, unhook_antigravity_api_handler,
    ProxyRuntimeState,
};
use super::sanitizer::IngressSecuritySanitizer;
use axum::{
    body::Body,
    http::StatusCode,
    middleware,
    response::{Html, Response},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

pub struct LocalProxyServer {
    bind_port: u16,
    runtime_state: ProxyRuntimeState,
}

async fn origin_guard_middleware(
    incoming_request: axum::extract::Request,
    next: middleware::Next,
) -> Result<Response, (StatusCode, &'static str)> {
    let headers = incoming_request.headers();
    let origin_str = headers.get("origin").and_then(|h| h.to_str().ok());
    let referer_str = headers.get("referer").and_then(|h| h.to_str().ok());
    let host_str = headers.get("host").and_then(|h| h.to_str().ok());

    if IngressSecuritySanitizer::is_browser_origin_forbidden(origin_str)
        || IngressSecuritySanitizer::is_browser_origin_forbidden(referer_str)
        || IngressSecuritySanitizer::is_host_header_forbidden(host_str)
    {
        return Err((
            StatusCode::FORBIDDEN,
            "Cross-Origin requests or DNS rebinding from external domains are strictly forbidden.",
        ));
    }

    Ok(next.run(incoming_request).await)
}

impl LocalProxyServer {
    #[must_use]
    pub fn new(bind_port: u16, runtime_state: ProxyRuntimeState) -> Self {
        Self {
            bind_port,
            runtime_state,
        }
    }

    fn build_proxy_router(runtime_state: ProxyRuntimeState) -> Router {
        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_origin(AllowOrigin::predicate(|origin_header_val, _| {
                let origin_str = origin_header_val.to_str().ok();
                !IngressSecuritySanitizer::is_browser_origin_forbidden(origin_str)
            }));

        Router::new()
            .route("/", get(ui_index_handler))
            .route("/index.html", get(ui_index_handler))
            .route("/style.css", get(ui_style_handler))
            .route("/app.js", get(ui_script_handler))
            .route("/favicon.svg", get(ui_favicon_handler))
            .route("/favicon.ico", get(ui_favicon_handler))
            .route("/logo.png", get(ui_logo_handler))
            .route("/v1/models", get(list_models_handler))
            .route("/v1/chat/completions", post(chat_completions_handler))
            .route("/status", get(metrics_handler))
            .route("/health", get(metrics_handler))
            .route("/api/hook-status", get(hook_status_api_handler))
            .route(
                "/api/interpreter/status",
                get(interpreter_status_api_handler),
            )
            .route(
                "/api/interpreter/toggle",
                post(toggle_interpreter_api_handler),
            )
            .route("/api/rules/inspect", get(inspect_rules_api_handler))
            .route("/api/hook", post(hook_antigravity_api_handler))
            .route("/api/unhook", post(unhook_antigravity_api_handler))
            .route("/api/purify", post(purify_prompt_api_handler))
            .route("/api/prune", post(prune_terminal_api_handler))
            .route("/api/startup", get(get_startup_api_handler))
            .route("/api/startup/toggle", post(toggle_startup_api_handler))
            .route("/api/gate/install", post(install_gate_api_handler))
            .route(
                "/api/project/export",
                post(export_project_config_api_handler),
            )
            .route("/api/workspace/default", get(default_workspace_api_handler))
            .route("/api/engine/status", get(engine_status_api_handler))
            .route("/api/engine/bootstrap", post(engine_bootstrap_api_handler))
            .route("/api/quit", post(quit_app_api_handler))
            .layer(middleware::from_fn(origin_guard_middleware))
            .layer(cors)
            .with_state(runtime_state)
    }

    pub async fn run_loopback_listener(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = Self::build_proxy_router(self.runtime_state);

        let socket_address: SocketAddr = format!("127.0.0.1:{}", self.bind_port).parse()?;
        let listener = match tokio::net::TcpListener::bind(socket_address).await {
            Ok(l) => l,
            Err(_) => {
                #[cfg(target_os = "windows")]
                launch_desktop_card_window(self.bind_port);
                return Ok(());
            }
        };

        tracing::info!(
            "GODKILLER ZERO active on http://{} (Strict Loopback)",
            socket_address
        );

        axum::serve(listener, app)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await?;

        Ok(())
    }

    async fn shutdown_signal() {
        let ctrl_c = async {
            let _ = tokio::signal::ctrl_c().await;
        };

        #[cfg(unix)]
        let terminate = async {
            if let Ok(mut signal_stream) =
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            {
                signal_stream.recv().await;
            }
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        tracing::info!("Shutting down GODKILLER ZERO loopback proxy gracefully...");
    }
}

async fn ui_index_handler() -> Html<&'static str> {
    Html(include_str!("../../src-ui/index.html"))
}

async fn ui_style_handler() -> Response {
    Response::builder()
        .header("Content-Type", "text/css; charset=utf-8")
        .body(Body::from(include_str!("../../src-ui/style.css")))
        .unwrap_or_default()
}

async fn ui_script_handler() -> Response {
    Response::builder()
        .header("Content-Type", "application/javascript; charset=utf-8")
        .body(Body::from(include_str!("../../src-ui/app.js")))
        .unwrap_or_default()
}

async fn ui_favicon_handler() -> Response {
    Response::builder()
        .header("Content-Type", "image/svg+xml; charset=utf-8")
        .body(Body::from(include_str!("../../src-ui/favicon.svg")))
        .unwrap_or_default()
}

async fn ui_logo_handler() -> Response {
    Response::builder()
        .header("Content-Type", "image/png")
        .body(Body::from(
            include_bytes!("../../src-ui/logo.png").as_slice(),
        ))
        .unwrap_or_default()
}

#[cfg(target_os = "windows")]
pub fn launch_desktop_card_window(_bind_port: u16) {
    let current_exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let native_gui_candidates = [
        current_exe_dir.join("GodkillerZero.exe"),
        std::path::PathBuf::from("GodkillerZero.exe"),
        current_exe_dir.join("publish").join("GodkillerZero.exe"),
        current_exe_dir
            .join("gui")
            .join("bin")
            .join("Release")
            .join("net9.0-windows")
            .join("win-x64")
            .join("GodkillerZero.exe"),
        current_exe_dir
            .join("gui")
            .join("bin")
            .join("Debug")
            .join("net9.0-windows")
            .join("GodkillerZero.exe"),
        current_exe_dir.join("GodkillerZeroGui.exe"),
        std::path::PathBuf::from("GodkillerZeroGui.exe"),
        current_exe_dir.join("publish").join("GodkillerZeroGui.exe"),
    ];

    for candidate in &native_gui_candidates {
        if candidate.exists() && std::process::Command::new(candidate).spawn().is_ok() {
            return;
        }
    }

    // Fallback: Launch embedded Zen UI via Edge App Mode
    let edge_app_arg = format!("--app=http://127.0.0.1:{}", _bind_port);
    let edge_paths = [
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
    ];
    for edge in &edge_paths {
        if std::path::Path::new(edge).exists() {
            let _ = std::process::Command::new(edge).arg(&edge_app_arg).spawn();
            return;
        }
    }
}
