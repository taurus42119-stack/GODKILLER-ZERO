#[cfg(target_os = "windows")]
use std::process::Command;

#[cfg(target_os = "windows")]
const STARTUP_REG_KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
#[cfg(target_os = "windows")]
const STARTUP_VALUE_NAME: &str = "GodkillerZero";

#[must_use]
pub fn query_windows_startup_status() -> bool {
    #[cfg(target_os = "windows")]
    {
        let execution_receipt = Command::new("reg")
            .args(["query", STARTUP_REG_KEY, "/v", STARTUP_VALUE_NAME])
            .output();

        match execution_receipt {
            Ok(process_output) => process_output.status.success(),
            Err(_) => false,
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

pub fn configure_windows_startup(should_enable: bool) -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        if should_enable {
            let binary_path =
                std::env::current_exe().map_err(|error_descriptor| error_descriptor.to_string())?;
            let formatted_command = format!("\"{}\" --minimized", binary_path.to_string_lossy());

            let execution_status = Command::new("reg")
                .args([
                    "add",
                    STARTUP_REG_KEY,
                    "/v",
                    STARTUP_VALUE_NAME,
                    "/t",
                    "REG_SZ",
                    "/d",
                    &formatted_command,
                    "/f",
                ])
                .status()
                .map_err(|error_descriptor| error_descriptor.to_string())?;

            Ok(execution_status.success())
        } else {
            if !query_windows_startup_status() {
                return Ok(true);
            }
            let execution_status = Command::new("reg")
                .args(["delete", STARTUP_REG_KEY, "/v", STARTUP_VALUE_NAME, "/f"])
                .status()
                .map_err(|error_descriptor| error_descriptor.to_string())?;

            Ok(execution_status.success() || !query_windows_startup_status())
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = should_enable;
        Ok(false)
    }
}
