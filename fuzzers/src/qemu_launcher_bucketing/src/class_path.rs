#[derive(PartialEq)]
pub enum PathRules {
    Env_resources,
    Exec_runtime,
    Unsure,
}

pub fn class_path(path: &str) -> PathRules {
    if path.starts_with("/proc") || path.starts_with("/sys/") || path.starts_with("/dev/") {
        PathRules::Env_resources
    } else if path.starts_with("/lib/") || path.starts_with(".so") || path.starts_with("/usr/bin") {
        PathRules::Exec_runtime
    } else {
        PathRules::Unsure
    }
}
