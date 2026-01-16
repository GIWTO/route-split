// 注册表操作模块：代理状态读取和设置

use winreg::enums::*;
use winreg::RegKey;

/// 注册表操作结果
#[derive(Debug)]
pub struct RegistryResult {
    pub success: bool,
    pub message: String,
}

/// Internet Settings 注册表路径
const INTERNET_SETTINGS_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";

/// 获取系统代理是否启用
pub fn get_proxy_enabled() -> Result<bool, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    match hkcu.open_subkey(INTERNET_SETTINGS_PATH) {
        Ok(key) => {
            match key.get_value::<u32, _>("ProxyEnable") {
                Ok(value) => Ok(value != 0),
                Err(_) => Ok(false), // 如果键不存在，默认为关闭
            }
        }
        Err(e) => Err(format!("Failed to open registry key: {}", e)),
    }
}

/// 设置系统代理开关
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

/// 获取当前代理服务器地址
pub fn get_proxy_server() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    if let Ok(key) = hkcu.open_subkey(INTERNET_SETTINGS_PATH) {
        if let Ok(server) = key.get_value::<String, _>("ProxyServer") {
            return Some(server);
        }
    }
    
    None
}
