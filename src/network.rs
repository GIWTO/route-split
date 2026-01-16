// 网络操作模块：权限检测、网关探测、路由操作

use std::process::Command;

/// 检测当前进程是否具有管理员权限
pub fn is_admin() -> bool {
    let output = Command::new("cmd").args(["/C", "net", "session"]).output();
    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}

/// 适配器详细信息
#[derive(Debug, Clone, PartialEq)]
pub struct AdapterInfo {
    pub name: String,
    pub ip: String,
    pub gateway: String,
}

/// 获取所有网卡的基本信息
pub fn get_all_adapters() -> Vec<AdapterInfo> {
    let mut adapters = Vec::new();

    // 强制使用 UTF-8 输出并运行 ipconfig
    let output = Command::new("cmd")
        .args(["/C", "chcp 65001 > nul && ipconfig"])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut current_name = String::new();
        let mut current_ip = String::new();
        let mut current_gw = String::new();

        for line in stdout.lines() {
            let line_raw = line.trim_end();
            let line_trimmed = line_raw.trim();

            if line_trimmed.is_empty() {
                continue;
            }

            // 识别网卡起始行：适配器名称通常位于行首，不以空格/制表符开头
            // 且通常包含 "adapter" 或 "适配器" 字样，或者仅仅是非缩进行
            if !line_raw.starts_with(' ') && !line_raw.starts_with('\t') {
                // 如果之前搜集到了有效信息，保存
                if !current_name.is_empty() && !current_ip.is_empty() {
                    adapters.push(AdapterInfo {
                        name: current_name.clone(),
                        ip: current_ip.clone(),
                        gateway: if current_gw.is_empty() {
                            "无网关".into()
                        } else {
                            current_gw.clone()
                        },
                    });
                }
                // 记录新适配器名称
                current_name = line_trimmed.trim_end_matches(':').to_string();
                current_ip.clear();
                current_gw.clear();
                continue;
            }

            // 匹配 IP 地址 (不区分中英文)
            if line_trimmed.contains("IPv4") && line_trimmed.contains(':') {
                if let Some(val) = line_trimmed.split(':').nth(1) {
                    current_ip = val.trim().to_string();
                }
            }

            // 匹配默认网关 (不区分中英文)
            if (line_trimmed.contains("Gateway") || line_trimmed.contains("网关"))
                && line_trimmed.contains(':')
            {
                if let Some(val) = line_trimmed.split(':').nth(1) {
                    let gw = val.trim();
                    if !gw.is_empty() && !gw.starts_with("::") {
                        // 过滤 IPv6
                        current_gw = gw.to_string();
                    }
                }
            }
        }

        // 保存最后一个
        if !current_name.is_empty() && !current_ip.is_empty() {
            adapters.push(AdapterInfo {
                name: current_name,
                ip: current_ip,
                gateway: if current_gw.is_empty() {
                    "无网关".into()
                } else {
                    current_gw
                },
            });
        }
    }

    // --- 兜底方案：如果 ipconfig 没拿到，尝试 PowerShell (仅在空列表时执行) ---
    if adapters.is_empty() {
        let ps_cmd = "Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.PrefixOrigin -ne 'WellKnown' } | Select-Object InterfaceAlias, IPAddress | ConvertTo-Json";
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", ps_cmd])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                if let Some(list) = json.as_array() {
                    for item in list {
                        let name = item["InterfaceAlias"]
                            .as_str()
                            .unwrap_or("未知网卡")
                            .to_string();
                        let ip = item["IPAddress"].as_str().unwrap_or("").to_string();
                        if !ip.is_empty() {
                            adapters.push(AdapterInfo {
                                name,
                                ip,
                                gateway: "待探测".into(),
                            });
                        }
                    }
                } else if let Some(item) = json.as_object() {
                    let name = item["InterfaceAlias"]
                        .as_str()
                        .unwrap_or("未知网卡")
                        .to_string();
                    let ip = item["IPAddress"].as_str().unwrap_or("").to_string();
                    adapters.push(AdapterInfo {
                        name,
                        ip,
                        gateway: "待探测".into(),
                    });
                }
            }
        }
    }

    adapters
}

/// 路由操作结果
pub struct RouteResult {
    pub success: bool,
    pub message: String,
}

/// 检查特定路由是否存在
pub fn check_route_exists(dest: &str) -> bool {
    // 强制 UTF8
    if let Ok(output) = Command::new("cmd")
        .args(["/C", "chcp 65001 > nul && route print", dest, "-4"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.contains(dest)
    } else {
        false
    }
}

/// 添加路由
pub fn add_route(dest: &str, mask: &str, gw: &str) -> RouteResult {
    if gw == "无网关" || gw == "待探测" || gw.is_empty() {
        return RouteResult {
            success: false,
            message: "该网卡没有有效网关，无法分流".into(),
        };
    }
    let _ = Command::new("cmd")
        .args(["/C", "route", "delete", dest])
        .output();
    let res = Command::new("cmd")
        .args([
            "/C", "route", "add", dest, "mask", mask, gw, "metric", "10", "-p",
        ])
        .output();
    match res {
        Ok(o) => RouteResult {
            success: o.status.success(),
            message: if o.status.success() {
                format!("已绑定到网关 {}", gw)
            } else {
                "添加失败".into()
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
    let _ = Command::new("cmd")
        .args(["/C", "route", "delete", dest])
        .output();
    RouteResult {
        success: true,
        message: "已处理".into(),
    }
}

/// 刷新 DNS
pub fn flush_network() -> RouteResult {
    let _ = Command::new("cmd")
        .args(["/C", "ipconfig", "/flushdns"])
        .output();
    RouteResult {
        success: true,
        message: "DNS 已刷新".into(),
    }
}
