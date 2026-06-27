// 网络操作模块：跨平台接口封装

use crate::platform::network as platform;

/// 适配器详细信息
#[derive(Debug, Clone, PartialEq)]
pub struct AdapterInfo {
    pub name: String,
    pub ip: String,
    pub gateway: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouteEntry {
    pub destination: String,
    pub mask: String,
    pub gateway: String,
    pub interface: String,
}

/// 路由操作结果
pub struct RouteResult {
    pub success: bool,
    pub message: String,
}

/// 检测当前进程是否具有管理员权限
pub fn is_admin() -> bool {
    platform::is_admin()
}

/// 获取所有网卡的基本信息
pub fn get_all_adapters() -> Vec<AdapterInfo> {
    platform::get_all_adapters()
}

/// 检查特定路由是否存在
pub fn check_route_exists(dest: &str) -> bool {
    platform::check_route_exists(dest)
}

/// 获取活跃的 IPv4 路由表条目
pub fn get_active_routes() -> Vec<RouteEntry> {
    platform::get_active_routes()
}

/// 过滤出关键的分流路由
pub fn get_active_routes_filtered(prefix: &str) -> Vec<RouteEntry> {
    get_active_routes()
        .into_iter()
        .filter(|r| r.destination.starts_with(prefix))
        .collect()
}

/// Ping 网关并返回延迟 (ms)
pub fn ping_gateway(ip: &str) -> Option<u32> {
    if ip == "无网关" || ip == "待探测" || ip.is_empty() {
        return None;
    }
    platform::ping_gateway(ip)
}

/// 添加路由
pub fn add_route(dest: &str, mask: &str, gw: &str) -> RouteResult {
    if gw == "无网关" || gw == "待探测" || gw.is_empty() {
        return RouteResult {
            success: false,
            message: "该网卡没有有效网关，无法分流".into(),
        };
    }
    platform::add_route(dest, mask, gw)
}

/// 删除路由
pub fn delete_route(dest: &str) -> RouteResult {
    platform::delete_route(dest)
}

/// 刷新网络 (DNS)
pub fn flush_network() -> RouteResult {
    platform::flush_network()
}
