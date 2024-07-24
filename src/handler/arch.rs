use crate::utils::system::SystemInfo;

pub fn handle_arch() {
    if SystemInfo::is_supported_system() {
        let sys_info = SystemInfo::new();
        println!(
            "System Info: {} {}",
            sys_info.sys_long_os_version, sys_info.sys_cpu_arch
        );
    } else {
        println!("This OS isn't supported (yet?).");
    }
}
