// Linux 平台注册表/代理 Mock 实现

use crate::registry::RegistryResult;

pub fn get_proxy_enabled() -> Result<bool, String> {
    // Linux 下通常检查环境变量，这里暂时简单适配
    if std::env::var("http_proxy").is_ok() || std::env::var("HTTP_PROXY").is_ok() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn set_proxy_enabled(_enabled: bool) -> RegistryResult {
    RegistryResult {
        success: false,
        message: "Linux 下暂不支持通过应用修改系统代理。".into(),
    }
}

pub fn get_proxy_server() -> Option<String> {
    std::env::var("http_proxy").ok().or_else(|| std::env::var("HTTP_PROXY").ok())
}
