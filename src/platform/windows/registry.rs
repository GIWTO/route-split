// Windows 平台注册表/代理实现

use winreg::enums::*;
use winreg::RegKey;
use crate::registry::RegistryResult;

/// Internet Settings 注册表路径
const INTERNET_SETTINGS_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";

pub fn get_proxy_enabled() -> Result<bool, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey(INTERNET_SETTINGS_PATH) {
        Ok(key) => {
            match key.get_value::<u32, _>("ProxyEnable") {
                Ok(value) => Ok(value != 0),
                Err(_) => Ok(false),
            }
        }
        Err(e) => Err(format!("Failed to open registry key: {}", e)),
    }
}

pub fn set_proxy_enabled(enabled: bool) -> RegistryResult {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey_with_flags(INTERNET_SETTINGS_PATH, KEY_WRITE) {
        Ok(key) => {
            let value: u32 = if enabled { 1 } else { 0 };
            match key.set_value("ProxyEnable", &value) {
                Ok(_) => RegistryResult {
                    success: true,
                    message: format!("Proxy {}", if enabled { "enabled" } else { "disabled" }),
                },
                Err(e) => RegistryResult {
                    success: false,
                    message: format!("Failed to set ProxyEnable: {}", e),
                },
            }
        }
        Err(e) => RegistryResult {
            success: false,
            message: format!("Access Denied: Registry - {}", e),
        },
    }
}

pub fn get_proxy_server() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey(INTERNET_SETTINGS_PATH) {
        if let Ok(server) = key.get_value::<String, _>("ProxyServer") {
            return Some(server);
        }
    }
    None
}
