use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// 查看当前系统架构和node架构
    Arch,
    /// 显示当前版本
    Current,
    /// 下载并安装指定版本
    Install,
    /// 卸载已下载的版本
    Uninstall,
    /// 查看当前已下载版本列表
    List,
    /// 设置下载代理。留空以查看当前代理
    Proxy { url: Option<String> },
    /// 使用指定版本
    Use { version: String },
}
