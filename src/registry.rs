// 注册表/代理操作模块：跨平台接口封装

use crate::platform::registry as platform;

/// 注册表操作结果
#[derive(Debug)]
pub struct RegistryResult {
    pub success: bool,
    pub message: String,
}

/// 获取系统代理是否启用
pub fn get_proxy_enabled() -> Result<bool, String> {
    platform::get_proxy_enabled()
}

/// 设置系统代理开关
pub fn set_proxy_enabled(enabled: bool) -> RegistryResult {
    platform::set_proxy_enabled(enabled)
}

/// 获取当前代理服务器地址
pub fn get_proxy_server() -> Option<String> {
    platform::get_proxy_server()
}
