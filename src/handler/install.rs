pub fn handle_install(version: String) {
    println!("Installing... {}", version);
    // 针对Linux macOS Windows 为不同的下载链接
    let download_url = format!(
        "https://nodejs.org/dist/v{}/node-v{}.tar.gz",
        version, version
    );
}
