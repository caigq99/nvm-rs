use crate::utils::system::SystemInfo;

pub const NODE_VERSION_JSON_URL: &str = "https://nodejs.org/dist/index.json";

const NODE_DOWNLOAD_URL: &str = "https://registry.npmmirror.com/-/binary/node/";

const NODE_DOWNLOAD_PREFIX: &str = "node-";
const NODE_DOWNLOAD_SUFFIX: &str = ".tar.gz";
const NODE_DOWNLOAD_SUFFIX_WIN: &str = ".zip";

pub fn get_suffix() -> &'static str {
    match SystemInfo::is_windows() {
        true => NODE_DOWNLOAD_SUFFIX_WIN,
        false => NODE_DOWNLOAD_SUFFIX,
    }
}

pub fn get_download_url(version: &str) -> String {
    format!(
        "{}{}/{}{}-{}-{}{}",
        NODE_DOWNLOAD_URL,
        version,
        NODE_DOWNLOAD_PREFIX,
        version,
        SystemInfo::get_system_name(),
        SystemInfo::get_cpu_arch(),
        get_suffix()
    )
}
