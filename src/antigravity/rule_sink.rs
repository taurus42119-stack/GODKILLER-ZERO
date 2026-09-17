use std::path::{Path, PathBuf};

const ZERO_MDC: &str = "godkiller-zero.mdc";

/// OS-agnostic roots. Production fills these from env; tests pass temp dirs
/// so discovery never depends on a specific user, drive letter, or locale.
#[derive(Debug, Clone)]
pub struct SinkProbeRoots {
    pub home: PathBuf,
    pub workspace: PathBuf,
    pub xdg_config: Option<PathBuf>,
    pub extra_cursor_roots: Vec<PathBuf>,
    pub extra_claude_roots: Vec<PathBuf>,
}

impl SinkProbeRoots {
    #[must_use]
    pub fn from_live_environment() -> Self {
        Self {
            home: resolve_user_home(),
            workspace: resolve_workspace_root(),
            xdg_config: std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from),
            extra_cursor_roots: env_dirs(&["CURSOR_HOME", "CURSOR_USER_DIR"]),
            extra_claude_roots: env_dirs(&["CLAUDE_CONFIG_DIR"]),
        }
    }
}

#[must_use]
pub fn discover_live_rule_sinks() -> Vec<PathBuf> {
    discover_rule_sinks(&SinkProbeRoots::from_live_environment())
}

#[must_use]
pub fn discover_live_legacy_cleanup_paths() -> Vec<PathBuf> {
    discover_legacy_cleanup_paths(&SinkProbeRoots::from_live_environment())
}

#[must_use]
pub fn discover_rule_sinks(roots: &SinkProbeRoots) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    push_cursor_sinks(&mut paths, roots);
    push_claude_sinks(&mut paths, roots);
    push_copilot_sinks(&mut paths, roots);
    dedup_paths(paths)
}

#[must_use]
pub fn discover_legacy_cleanup_paths(roots: &SinkProbeRoots) -> Vec<PathBuf> {
    vec![
        roots.home.join(".cursorrules"),
        roots.workspace.join(".cursorrules"),
    ]
}

fn resolve_user_home() -> PathBuf {
    if let Ok(profile) = std::env::var("USERPROFILE") {
        if !profile.is_empty() {
            return PathBuf::from(profile);
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return PathBuf::from(home);
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn resolve_workspace_root() -> PathBuf {
    let Ok(start) = std::env::current_dir() else {
        return PathBuf::from(".");
    };
    find_project_root(&start).unwrap_or(start)
}

fn env_dirs(keys: &[&str]) -> Vec<PathBuf> {
    keys.iter()
        .filter_map(|key| std::env::var_os(key).map(PathBuf::from))
        .filter(|path| !path.as_os_str().is_empty())
        .collect()
}

fn find_project_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if is_project_root(&current) {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

fn is_project_root(dir: &Path) -> bool {
    dir.join(".git").exists()
        || dir.join("Cargo.toml").is_file()
        || dir.join("package.json").is_file()
        || dir.join(".cursor").is_dir()
        || dir.join(".gemini").is_dir()
        || dir.join(".claude").is_dir()
        || dir.join(".agents").is_dir()
}

fn push_cursor_sinks(paths: &mut Vec<PathBuf>, roots: &SinkProbeRoots) {
    let mut host_dirs = roots.extra_cursor_roots.clone();
    push_if_dir(&mut host_dirs, roots.home.join(".cursor"));
    if let Some(xdg) = &roots.xdg_config {
        push_if_dir(&mut host_dirs, xdg.join("cursor"));
        push_if_dir(&mut host_dirs, xdg.join("Cursor"));
    }
    push_if_dir(&mut host_dirs, roots.workspace.join(".cursor"));

    let cursor_in_use = !host_dirs.is_empty();
    for host_dir in &host_dirs {
        paths.push(host_dir.join("rules").join(ZERO_MDC));
    }
    if cursor_in_use && is_project_root(&roots.workspace) {
        let project_rules = roots.workspace.join(".cursor").join("rules").join(ZERO_MDC);
        paths.push(project_rules);
    }
}

fn push_claude_sinks(paths: &mut Vec<PathBuf>, roots: &SinkProbeRoots) {
    let mut host_dirs = roots.extra_claude_roots.clone();
    push_if_dir(&mut host_dirs, roots.home.join(".claude"));
    if let Some(xdg) = &roots.xdg_config {
        push_if_dir(&mut host_dirs, xdg.join("claude"));
    }
    push_if_dir(&mut host_dirs, roots.workspace.join(".claude"));

    for host_dir in &host_dirs {
        paths.push(host_dir.join("CLAUDE.md"));
    }
    if roots.workspace.join("CLAUDE.md").is_file() {
        paths.push(roots.workspace.join("CLAUDE.md"));
    }
}

fn push_copilot_sinks(paths: &mut Vec<PathBuf>, roots: &SinkProbeRoots) {
    let github = roots.workspace.join(".github");
    if github.is_dir() || roots.workspace.join(".git").exists() {
        paths.push(github.join("copilot-instructions.md"));
    }
}

fn push_if_dir(out: &mut Vec<PathBuf>, candidate: PathBuf) {
    if candidate.is_dir() {
        out.push(candidate);
    }
}

fn dedup_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut unique = Vec::new();
    for path in paths {
        if !unique.iter().any(|existing: &PathBuf| existing == &path) {
            unique.push(path);
        }
    }
    unique
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn make_temp_tree(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!("gk-sink-{label}-{nanos}"));
        let _ = fs::create_dir_all(&root);
        root
    }

    fn empty_roots(home: PathBuf, workspace: PathBuf) -> SinkProbeRoots {
        SinkProbeRoots {
            home,
            workspace,
            xdg_config: None,
            extra_cursor_roots: Vec::new(),
            extra_claude_roots: Vec::new(),
        }
    }

    #[test]
    fn cursor_uses_native_mdc_not_legacy_cursorrules() {
        let home = make_temp_tree("home");
        let workspace = make_temp_tree("ws");
        fs::create_dir_all(home.join(".cursor")).unwrap();
        fs::create_dir_all(workspace.join(".git")).unwrap();

        let sinks = discover_rule_sinks(&empty_roots(home.clone(), workspace.clone()));
        let rendered: Vec<String> = sinks
            .iter()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .collect();

        assert!(
            rendered
                .iter()
                .any(|p| p.ends_with(".cursor/rules/godkiller-zero.mdc")),
            "expected Cursor native mdc, got {rendered:?}"
        );
        assert!(
            !rendered.iter().any(|p| p.ends_with(".cursorrules")),
            "legacy ~/.cursorrules must not be a write target: {rendered:?}"
        );

        let _ = fs::remove_dir_all(home);
        let _ = fs::remove_dir_all(workspace);
    }

    #[test]
    fn linux_xdg_cursor_is_discovered() {
        let home = make_temp_tree("xdg-home");
        let workspace = make_temp_tree("xdg-ws");
        let xdg = make_temp_tree("xdg-config");
        fs::create_dir_all(xdg.join("cursor")).unwrap();

        let mut roots = empty_roots(home.clone(), workspace.clone());
        roots.xdg_config = Some(xdg.clone());
        let sinks = discover_rule_sinks(&roots);
        let rendered: Vec<String> = sinks
            .iter()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .collect();

        assert!(
            rendered
                .iter()
                .any(|p| p.contains("/cursor/rules/godkiller-zero.mdc")),
            "expected XDG cursor sink, got {rendered:?}"
        );

        let _ = fs::remove_dir_all(home);
        let _ = fs::remove_dir_all(workspace);
        let _ = fs::remove_dir_all(xdg);
    }

    #[test]
    fn claude_config_dir_env_is_honored() {
        let home = make_temp_tree("claude-home");
        let workspace = make_temp_tree("claude-ws");
        let claude = make_temp_tree("claude-config");

        let mut roots = empty_roots(home.clone(), workspace.clone());
        roots.extra_claude_roots = vec![claude.clone()];
        let sinks = discover_rule_sinks(&roots);

        assert!(sinks.iter().any(|p| p == &claude.join("CLAUDE.md")));

        let _ = fs::remove_dir_all(home);
        let _ = fs::remove_dir_all(workspace);
        let _ = fs::remove_dir_all(claude);
    }

    #[test]
    fn unknown_machine_without_host_dirs_writes_nothing() {
        let home = make_temp_tree("empty-home");
        let workspace = make_temp_tree("empty-ws");
        let sinks = discover_rule_sinks(&empty_roots(home.clone(), workspace.clone()));
        assert!(
            sinks.is_empty(),
            "no host markers must not invent paths: {sinks:?}"
        );
        let _ = fs::remove_dir_all(home);
        let _ = fs::remove_dir_all(workspace);
    }

    #[test]
    fn legacy_cleanup_lists_dead_cursorrules_only_for_unhook() {
        let home = PathBuf::from("/tmp/someone");
        let workspace = PathBuf::from("/Users/someone/project");
        let cleanup = discover_legacy_cleanup_paths(&empty_roots(home, workspace));
        assert_eq!(cleanup.len(), 2);
        assert!(cleanup[0].ends_with(".cursorrules"));
        assert!(cleanup[1].ends_with(".cursorrules"));
    }
}
