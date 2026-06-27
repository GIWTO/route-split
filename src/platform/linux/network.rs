// Linux 平台网络操作实现

use std::process::Command;
use crate::network::{AdapterInfo, RouteEntry, RouteResult};

/// 检测当前进程是否具有管理员权限 (Root)
pub fn is_admin() -> bool {
    let output = Command::new("id").arg("-u").output();
    if let Ok(o) = output {
        let stdout = String::from_utf8_lossy(&o.stdout).trim().to_string();
        stdout == "0"
    } else {
        false
    }
}

/// 获取所有网卡的基本信息
pub fn get_all_adapters() -> Vec<AdapterInfo> {
    let mut adapters = Vec::new();
    
    // 运行 ip addr 获取网卡信息
    let output = Command::new("ip").arg("addr").output();
    
    if let Ok(o) = output {
        let stdout = String::from_utf8_lossy(&o.stdout);
        let mut current_name = String::new();
        let mut current_ip = String::new();
        
        for line in stdout.lines() {
            let trimmed = line.trim();
            // 匹配行首的数字编号，例如 "2: eth0: <BROADCAST...>"
            if !line.starts_with(' ') && line.contains(':') {
                // 保存上一个
                if !current_name.is_empty() && !current_ip.is_empty() {
                    adapters.push(AdapterInfo {
                        name: current_name.clone(),
                        ip: current_ip.clone(),
                        gateway: "待探测".into(),
                    });
                }
                
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    current_name = parts[1].trim().to_string();
                }
                current_ip.clear();
                continue;
            }
            
            // 匹配 IPv4 地址
            if trimmed.starts_with("inet ") {
                if let Some(ip_with_mask) = trimmed.split_whitespace().nth(1) {
                    current_ip = ip_with_mask.split('/').next().unwrap_or("").to_string();
                }
            }
        }
        
        // 保存最后一个
        if !current_name.is_empty() && !current_ip.is_empty() {
            adapters.push(AdapterInfo {
                name: current_name,
                ip: current_ip,
                gateway: "待探测".into(),
            });
        }
    }
    
    adapters
}

/// 检查特定路由是否存在
pub fn check_route_exists(dest: &str) -> bool {
    let output = Command::new("ip").args(["route", "show", dest]).output();
    if let Ok(o) = output {
        let stdout = String::from_utf8_lossy(&o.stdout);
        !stdout.trim().is_empty()
    } else {
        false
    }
}

/// 获取活跃的 IPv4 路由表条目
pub fn get_active_routes() -> Vec<RouteEntry> {
    let mut routes = Vec::new();
    let output = Command::new("ip").args(["route", "show"]).output();
    
    if let Ok(o) = output {
        let stdout = String::from_utf8_lossy(&o.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // 示例: "default via 192.168.1.1 dev eth0 proto dhcp metric 100"
            // 示例: "10.0.0.0/24 dev eth1 proto kernel scope link src 10.0.0.5"
            
            let destination = parts.get(0).unwrap_or(&"").to_string();
            let mut gateway = "0.0.0.0".to_string();
            let mut interface = "unknown".to_string();
            
            for (i, part) in parts.iter().enumerate() {
                if *part == "via" {
                    if let Some(next) = parts.get(i + 1) {
                        gateway = next.to_string();
                    }
                }
                if *part == "dev" {
                    if let Some(next) = parts.get(i + 1) {
                        interface = next.to_string();
                    }
                }
            }
            
            routes.push(RouteEntry {
                destination,
                mask: "".into(), // Linux 路由通常目的地带掩码
                gateway,
                interface,
            });
        }
    }
    routes
}

/// Ping 网关并返回延迟 (ms)
pub fn ping_gateway(ip: &str) -> Option<u32> {
    // Linux 下使用 -c 1 (次数) 和 -W 1 (超时秒)
    let output = Command::new("ping")
        .args(["-c", "1", "-W", "1", ip])
        .output();
    
    if let Ok(o) = output {
        let stdout = String::from_utf8_lossy(&o.stdout);
        for line in stdout.lines() {
            if line.contains("time=") {
                let parts: Vec<&str> = line.split("time=").collect();
                if let Some(val_str) = parts.get(1) {
                    let val = val_str.split_whitespace().next().unwrap_or("");
                    if let Ok(f) = val.parse::<f32>() {
                        return Some(f.round() as u32);
                    }
                }
            }
        }
    }
    None
}

/// 添加路由
pub fn add_route(dest: &str, _mask: &str, gw: &str) -> RouteResult {
    // Linux 下通常直接使用 CIDR，如果 dest 不包含 /，可以尝试结合 mask
    // 但此处为了简单，假设 dest 已经包含了前缀
    let res = Command::new("ip")
        .args(["route", "add", dest, "via", gw])
        .output();
    
    match res {
        Ok(o) => RouteResult {
            success: o.status.success(),
            message: if o.status.success() {
                format!("已绑定到网关 {}", gw)
            } else {
                String::from_utf8_lossy(&o.stderr).trim().to_string()
            },
        },
        Err(e) => RouteResult {
            success: false,
            message: e.to_string(),
        },
    }
}

/// 删除路由
pub fn delete_route(dest: &str) -> RouteResult {
    let res = Command::new("ip")
        .args(["route", "del", dest])
        .output();
    
    match res {
        Ok(o) => RouteResult {
            success: o.status.success(),
            message: if o.status.success() {
                "已删除".into()
            } else {
                String::from_utf8_lossy(&o.stderr).trim().to_string()
            },
        },
        Err(e) => RouteResult {
            success: false,
            message: e.to_string(),
        },
    }
}

/// 刷新 DNS
pub fn flush_network() -> RouteResult {
    // 尝试多种常见的刷新方法
    let _ = Command::new("resolvectl").arg("flush-caches").output();
    let _ = Command::new("systemd-resolve").arg("--flush-caches").output();
    
    RouteResult {
        success: true,
        message: "已尝试刷新 DNS 缓存".into(),
    }
}
