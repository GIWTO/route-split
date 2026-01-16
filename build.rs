// 构建脚本：嵌入 Windows manifest 以请求管理员权限
fn main() {
    // 仅在 Windows 平台编译时执行
    #[cfg(windows)]
    {
        embed_resource::compile("app.rc", embed_resource::NONE);
    }
}
