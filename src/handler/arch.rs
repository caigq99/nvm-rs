use sysinfo::{self, System};

#[derive(Debug)]
pub struct SystemInfo {
    pub sys_name: String,
    pub sys_kernel_version: String,
    pub sys_os_version: String,
    pub sys_host_name: String,
    pub sys_cpu_arch: String,
    pub sys_long_os_version: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            sys_name: System::name().unwrap(),
            sys_kernel_version: System::kernel_version().unwrap(),
            sys_os_version: System::os_version().unwrap(),
            sys_host_name: System::host_name().unwrap(),
            sys_cpu_arch: System::cpu_arch().unwrap(),
            sys_long_os_version: System::long_os_version().unwrap(),
        }
    }
}

pub fn handle_arch() {
    if sysinfo::IS_SUPPORTED_SYSTEM {
        let sys_info = SystemInfo::new();
        println!(
            "System Info: {} {}",
            sys_info.sys_long_os_version, sys_info.sys_cpu_arch
        );
    } else {
        println!("This OS isn't supported (yet?).");
    }
}
