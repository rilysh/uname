#[cfg(target_os = "linux")]
pub(crate) struct Utsname {
    // Operating system name
    pub sysname: String,
    // Host name
    pub nodename: String,
    // Operating system release
    pub release: String,
    // Operating system version
    pub version: String,
    // Hardware type
    pub machine: String,
    // NIS or YP domain name (if available)
    pub domainname: String,
}

#[cfg(any(
    target_os = "freebsd",
    target_os = "dragonflybsd",
    target_os = "openbsd",
    target_os = "netbsd"))]
pub(crate) struct Utsname {
    // Operating system name
    pub sysname: String,
    // Host name
    pub nodename: String,
    // Operating system release
    pub release: String,
    // Operating system version
    pub version: String,
    // Hardware type
    pub machine: String,
}

