fn main() {
    // 告知 Cargo 在这些文件改变时重新运行构建脚本
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=app.rc");
    println!("cargo:rerun-if-changed=app.manifest");

    // 仅在 Windows 平台编译时执行
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        #[cfg(windows)]
        {
            embed_resource::compile("app.rc", embed_resource::NONE);
        }
    }
}
