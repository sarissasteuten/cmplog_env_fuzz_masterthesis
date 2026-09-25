use crate::hooks_harness::HARNESS_READY;
use std::sync::atomic::Ordering;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum PathRules {
    Env_resources,
    Exec_runtime,
    Unsure,
}

pub fn class_path(path: &str) -> PathRules {
    // eprintln!("classify_path: '{}'", path);
    
     if path == "/proc/self/exe" {
        if !HARNESS_READY.load(Ordering::Relaxed){
            return PathRules::Exec_runtime
        }
        PathRules::Env_resources 
    }else if path.starts_with("/proc/net/"){
        PathRules::Exec_runtime
    }else if path.starts_with("/proc") || path.starts_with("/sys/") || path.starts_with("/dev") {
        PathRules::Env_resources
    } else if path.starts_with("/lib/") || path.starts_with(".so") || path.starts_with("/usr/bin") {
        PathRules::Exec_runtime
    } else {
        PathRules::Unsure
    }
}