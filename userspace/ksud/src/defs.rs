use const_format::concatcp;

pub const ADB_DIR: &str = "/data/adb/";
pub const WORKING_DIR: &str = concatcp!(ADB_DIR, "ksu/");
pub const BINARY_DIR: &str = concatcp!(WORKING_DIR, "bin/");
pub const LOG_DIR: &str = concatcp!(WORKING_DIR, "log/");

pub const PROFILE_DIR: &str = concatcp!(WORKING_DIR, "profile/");
pub const PROFILE_SELINUX_DIR: &str = concatcp!(PROFILE_DIR, "selinux/");
pub const PROFILE_TEMPLATE_DIR: &str = concatcp!(PROFILE_DIR, "templates/");

pub const KSURC_PATH: &str = concatcp!(WORKING_DIR, ".ksurc");
pub const DAEMON_PATH: &str = concatcp!(ADB_DIR, "ksud");

#[cfg(target_os = "android")]
pub const DAEMON_LINK_PATH: &str = concatcp!(BINARY_DIR, "ksud");

pub const COMMON_DIR: &str = concatcp!(ADB_DIR, "common/");

pub const VERSION_CODE: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION_CODE"));
pub const VERSION_NAME: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION_NAME"));
