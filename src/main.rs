#![windows_subsystem = "windows"]

use clap::Parser;
use godkiller_zero::antigravity::{hook_antigravity, is_antigravity_hooked, unhook_antigravity};
use godkiller_zero::domain::AntiSpaghettiDirective;
#[cfg(target_os = "windows")]
use godkiller_zero::proxy::launch_desktop_card_window;
use godkiller_zero::proxy::{LocalProxyServer, ProxyRuntimeState};
use godkiller_zero::upstream::LocalZeroEngine;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

#[cfg(target_os = "windows")]
unsafe fn attach_parent_console_if_cli() {
    #[link(name = "kernel32")]
    extern "system" {
        fn AttachConsole(dwProcessId: u32) -> i32;
        fn GetStdHandle(nStdHandle: u32) -> isize;
        fn GetFileType(hfile: isize) -> u32;
    }
    const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5;
    const FILE_TYPE_PIPE: u32 = 0x0003;
    const FILE_TYPE_DISK: u32 = 0x0001;

    let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
    let file_type = GetFileType(stdout_handle);
    if file_type == FILE_TYPE_PIPE || file_type == FILE_TYPE_DISK {
        return;
    }

    let _ = AttachConsole(0xFFFFFFFF);
}

#[derive(Parser, Debug)]
#[command(
    name = "godkiller-zero",
    version = "1.0.0",
    about = "⚡ GODKILLER ZERO: Cognitive Pre-flight Firewall & Semantic IR Compiler for Google Antigravity"
)]
struct CliArguments {
    #[arg(short, long, default_value_t = 4242)]
    port: u16,

    #[arg(short, long, default_value = "KEN")]
    discipline: String,

    #[arg(long)]
    hook: bool,

    #[arg(long)]
    unhook: bool,

    #[arg(long, visible_alias = "simulate")]
    purify: Option<String>,

    #[arg(long)]
    gate: Option<Option<String>>,

    #[arg(long)]
    install_hook: Option<Option<String>>,

    #[arg(long)]
    mcp: bool,

    #[arg(long)]
    repo_map: Option<Option<String>>,

    #[arg(long, num_args = 1.., trailing_var_arg = true, allow_hyphen_values = true)]
    run: Option<Vec<String>>,

    #[arg(long, allow_hyphen_values = true)]
    prune: Option<String>,

    #[arg(long, default_value_t = false)]
    no_open: bool,

    #[arg(long, default_value_t = false)]
    minimized: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_mcp = std::env::args().any(|arg| arg == "--mcp");

    #[cfg(target_os = "windows")]
    if !is_mcp {
        unsafe {
            attach_parent_console_if_cli();
        }
    }

    let arguments = CliArguments::parse();

    if let Some(cmd_result) = dispatch_cli_command(&arguments) {
        return cmd_result;
    }

    run_server_mode(&arguments).await
}

fn dispatch_cli_command(
    arguments: &CliArguments,
) -> Option<Result<(), Box<dyn std::error::Error>>> {
    if arguments.mcp {
        return Some(godkiller_zero::proxy::McpServer::run_stdio_loop().map_err(|e| e.into()));
    }
    if let Some(target_dir_opt) = &arguments.install_hook {
        return Some(run_install_hook_command(target_dir_opt.clone()));
    }
    if let Some(target_path_opt) = &arguments.gate {
        return Some(run_gate_command(target_path_opt.clone(), &arguments.discipline));
    }
    dispatch_secondary_commands(arguments)
}

fn dispatch_secondary_commands(
    arguments: &CliArguments,
) -> Option<Result<(), Box<dyn std::error::Error>>> {
    if arguments.hook {
        return Some(run_hook_command(&arguments.discipline));
    }
    if arguments.unhook {
        return Some(run_unhook_command());
    }
    if let Some(raw_prompt_text) = &arguments.purify {
        return Some(run_purify_command(raw_prompt_text.clone(), &arguments.discipline));
    }
    if let Some(target_dir_opt) = &arguments.repo_map {
        return Some(run_repo_map_command(target_dir_opt.clone()));
    }
    if let Some(cmd_args) = &arguments.run {
        return Some(run_terminal_command(cmd_args));
    }
    arguments.prune.as_ref().map(|raw_text| run_prune_command(raw_text))
}

fn run_repo_map_command(
    target_dir_opt: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir_str = target_dir_opt.unwrap_or_else(|| ".".to_string());
    let target_dir = std::path::Path::new(&target_dir_str);
    let report = godkiller_zero::domain::RepoMapGenerator::generate(target_dir, 2048);
    println!("{}", report.map_content);
    println!(
        "\n✅ [REPO MAP GENERATED] Total files: {}, Symbols indexed: {}, Estimated tokens: {}",
        report.total_files, report.total_symbols, report.estimated_tokens
    );
    if let Some(ref path) = report.output_file_path {
        println!("Exported to: {}", path.display());
    }
    Ok(())
}

fn build_discipline_directive(discipline: &str) -> AntiSpaghettiDirective {
    match discipline.to_uppercase().as_str() {
        "SHI" => AntiSpaghettiDirective {
            max_cyclomatic_complexity: 10,
            enforce_disk_verification: false,
            ..Default::default()
        },
        "SHIN" => AntiSpaghettiDirective {
            max_cyclomatic_complexity: 5,
            enforce_disk_verification: true,
            ..Default::default()
        },
        _ => AntiSpaghettiDirective::default(),
    }
}

fn run_install_hook_command(
    target_dir_opt: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir_str = target_dir_opt.unwrap_or_else(|| ".".to_string());
    let target_dir = std::path::Path::new(&target_dir_str);
    match godkiller_zero::domain::GatekeeperScanner::install_git_pre_commit_hook(target_dir) {
        Ok(hook_path) => {
            println!("🛡️  [GODKILLER ZERO : GATEKEEPER INSTALLED]");
            println!("    Pre-commit hook installed at: {}", hook_path.display());
            Ok(())
        }
        Err(install_failure) => {
            eprintln!("❌ Failed to install Git pre-commit hook: {}", install_failure);
            std::process::exit(1);
        }
    }
}

fn run_gate_command(
    target_path_opt: Option<String>,
    discipline: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let scan_path_str = target_path_opt.unwrap_or_else(|| ".".to_string());
    let scan_path = std::path::Path::new(&scan_path_str);
    let (span_threshold, complexity_threshold) = match discipline.to_uppercase().as_str() {
        "SHI" => (90, 10),
        "SHIN" => (50, 5),
        _ => (70, 7),
    };

    println!("🛡️  [GODKILLER ZERO : RUNNING DISK GATEKEEPER]");
    println!("    Scanning target path: {}", scan_path.display());
    println!("    Function span budget: <= {} lines", span_threshold);
    println!("    Cognitive complexity budget: <= {}", complexity_threshold);

    let audit = godkiller_zero::domain::GatekeeperScanner::scan_path(
        scan_path,
        span_threshold,
        complexity_threshold,
    );
    if audit.passed {
        println!("\n{}", audit.summary_message);
        return Ok(());
    }

    eprintln!("\n{}", audit.summary_message);
    for violation in &audit.violations {
        eprintln!(
            "  • [{}] {}:L{}",
            violation.rule_identifier, violation.file_path, violation.line_number
        );
        eprintln!("    {} -> '{}'", violation.description, violation.snippet);
    }
    std::process::exit(1);
}

fn run_hook_command(discipline: &str) -> Result<(), Box<dyn std::error::Error>> {
    match hook_antigravity(discipline) {
        Ok(hook_receipt) => {
            println!("⛩️  [GODKILLER ZERO : HOOK SUCCESSFUL]");
            println!("    File Modified: {}", hook_receipt.path_modified.display());
            println!("    {}", hook_receipt.message);
            println!("\n    Google Antigravity IDE & CLI are now shielded with Zero-Vibe Invariants.");
            Ok(())
        }
        Err(hook_failure) => {
            eprintln!("❌ Failed to hook into Antigravity: {}", hook_failure);
            std::process::exit(1);
        }
    }
}

fn run_unhook_command() -> Result<(), Box<dyn std::error::Error>> {
    match unhook_antigravity() {
        Ok(unhook_receipt) => {
            println!("⛩️  [GODKILLER ZERO : UNHOOK SUCCESSFUL]");
            println!("    {}", unhook_receipt.message);
            Ok(())
        }
        Err(rollback_failure) => {
            eprintln!("❌ Failed to unhook from Antigravity: {}", rollback_failure);
            std::process::exit(1);
        }
    }
}

fn run_purify_command(
    raw_prompt_text: String,
    _discipline: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let transpiled = godkiller_zero::domain::LinguisticTranspiler::transpile(&raw_prompt_text);
    println!("{}", transpiled.concise_english);
    Ok(())
}

fn run_terminal_command(cmd_args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if cmd_args.is_empty() {
        eprintln!("❌ No command specified for --run. Example: godkiller-zero --run cargo test");
        std::process::exit(1);
    }

    let program = &cmd_args[0];
    let args = &cmd_args[1..];

    let output = match std::process::Command::new(program).args(args).output() {
        Ok(output) => output,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound && cfg!(target_os = "windows") => {
            // Fallback for Windows shell builtins / batch files (.cmd, .bat)
            std::process::Command::new("cmd")
                .arg("/C")
                .args(cmd_args)
                .output()?
        }
        Err(e) => return Err(format!("Failed to execute '{}': {}", program, e).into()),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let full_raw = if stderr.is_empty() {
        stdout.to_string()
    } else if stdout.is_empty() {
        stderr.to_string()
    } else {
        format!("{}\n{}", stdout, stderr)
    };

    let prune_result = godkiller_zero::domain::TerminalPruner::prune(&full_raw);
    print!("{}", prune_result.pruned_content);
    if !prune_result.pruned_content.ends_with('\n') {
        println!();
    }

    eprintln!(
        "\n⚡ [GODKILLER ZERO : TERMINAL SHIELD] Input: ~{} tokens | Shielded: ~{} tokens | Reduction: {:.1}%",
        prune_result.original_tokens_est,
        prune_result.pruned_tokens_est,
        prune_result.reduction_percentage
    );

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }
    Ok(())
}

fn run_prune_command(raw_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input = if raw_text == "-" || raw_text.trim().is_empty() {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        raw_text.to_string()
    };

    let prune_result = godkiller_zero::domain::TerminalPruner::prune(&input);
    print!("{}", prune_result.pruned_content);
    if !prune_result.pruned_content.ends_with('\n') {
        println!();
    }

    eprintln!(
        "\n⚡ [GODKILLER ZERO : TERMINAL SHIELD] Input: ~{} tokens | Shielded: ~{} tokens | Reduction: {:.1}%",
        prune_result.original_tokens_est,
        prune_result.pruned_tokens_est,
        prune_result.reduction_percentage
    );

    Ok(())
}

async fn run_server_mode(
    arguments: &CliArguments,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "godkiller_zero=info,tower_http=warn".into()),
        )
        .init();

    let is_shielded = is_antigravity_hooked();
    print_zen_banner(arguments.port, &arguments.discipline, is_shielded);

    let bootstrap_manager = Arc::new(godkiller_zero::upstream::OllamaBootstrapManager::new(
        "qwen2.5-coder:1.5b",
    ));
    bootstrap_manager.clone().start_background_bootstrap();

    let runtime_state = ProxyRuntimeState {
        upstream_gateway: Arc::new(LocalZeroEngine::new(Some(bootstrap_manager.clone()))),
        anti_spaghetti_directive: build_discipline_directive(&arguments.discipline),
        tokens_preserved_counter: Arc::new(AtomicU64::new(0)),
        halts_enforced_counter: Arc::new(AtomicU64::new(0)),
        requests_intercepted_counter: Arc::new(AtomicU64::new(0)),
        interpreter_active: Arc::new(std::sync::atomic::AtomicBool::new(true)),
        bootstrap_manager: Some(bootstrap_manager),
    };

    let proxy_server = LocalProxyServer::new(arguments.port, runtime_state);

    #[cfg(target_os = "windows")]
    let _tray = setup_system_tray(arguments.port);

    if !arguments.no_open && !arguments.minimized {
        let bind_port = arguments.port;
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(350)).await;
            #[cfg(target_os = "windows")]
            launch_desktop_card_window(bind_port);
        });
    }

    if let Err(err) = proxy_server.run_loopback_listener().await {
        tracing::error!("Proxy loopback listener terminated with error: {}", err);
        return Err(err);
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn load_custom_ico_from_path(ico_path: &std::path::Path) -> Option<isize> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::UI::WindowsAndMessaging::{LoadImageW, IMAGE_ICON, LR_LOADFROMFILE};
    if !ico_path.exists() {
        return None;
    }
    let mut wide_path: Vec<u16> = ico_path.as_os_str().encode_wide().collect();
    wide_path.push(0);
    let loaded = unsafe { LoadImageW(0, wide_path.as_ptr(), IMAGE_ICON, 32, 32, LR_LOADFROMFILE) };
    if loaded != 0 {
        Some(loaded)
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
fn find_custom_ico_handle() -> Option<isize> {
    let current_exe = std::env::current_exe().ok()?;
    let parent_dir = current_exe.parent()?;
    let candidates = [
        parent_dir.join("app.ico"),
        parent_dir.join("publish").join("app.ico"),
        std::path::PathBuf::from("app.ico"),
    ];
    candidates.iter().find_map(|p| load_custom_ico_from_path(p))
}

#[cfg(target_os = "windows")]
fn resolve_fallback_shield_icon() -> isize {
    use windows_sys::Win32::UI::WindowsAndMessaging::{LoadIconW, IDI_APPLICATION, IDI_SHIELD};
    unsafe {
        let shield = LoadIconW(0, IDI_SHIELD);
        if shield != 0 {
            shield
        } else {
            LoadIconW(0, IDI_APPLICATION)
        }
    }
}

#[cfg(target_os = "windows")]
fn setup_system_tray(bind_port: u16) -> Option<tray_item::TrayItem> {
    use tray_item::{IconSource, TrayItem};
    let app_icon = find_custom_ico_handle().unwrap_or_else(resolve_fallback_shield_icon);
    let mut system_tray = TrayItem::new("GODKILLER ZERO", IconSource::RawIcon(app_icon)).ok()?;
    let _ = system_tray.add_label("GODKILLER ZERO (v1.0.0)");
    let _ = system_tray.add_menu_item("Open Controller", move || {
        launch_desktop_card_window(bind_port);
    });
    let _ = system_tray.add_menu_item("Quit", move || {
        std::process::exit(0);
    });
    Some(system_tray)
}

fn print_zen_banner(port: u16, discipline: &str, is_shielded: bool) {
    let hook_status_label = if is_shielded {
        "SHIELDED (Injected in GEMINI.md)"
    } else {
        "NOT HOOKED (Run --hook or click UI)"
    };

    println!(
        r#"
  ┌────────────────────────────────────────────────────────┐
  │  零  GODKILLER ZERO                          v1.0.0    │
  │  Cognitive Pre-flight Firewall for Google Antigravity  │
  ├────────────────────────────────────────────────────────┤
  │  • Loopback Endpoint:  http://127.0.0.1:{:<5}         │
  │  • Discipline Mode:    {:<32}│
  │  • Antigravity IDE:    {:<32}│
  │  • Target Runtimes:    Antigravity IDE / CLI (agy)     │
  │  • Zero-Vibe Defense:  ACTIVE (Negative Invariants On) │
  └────────────────────────────────────────────────────────┘
"#,
        port, discipline, hook_status_label
    );
}
