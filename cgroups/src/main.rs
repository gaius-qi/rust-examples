use cgroups_rs::fs::{Cgroup, cpu::CpuController, hierarchies, memory::MemController};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeEnvironment {
    Container,
    Standalone,
}

pub fn detect_runtime_environment() -> RuntimeEnvironment {
    if is_container() {
        return RuntimeEnvironment::Container;
    }

    RuntimeEnvironment::Standalone
}

fn is_container() -> bool {
    if Path::new("/.dockerenv").exists() {
        return true;
    }

    if Path::new("/run/.containerenv").exists() {
        return true;
    }

    if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
        return true;
    }

    if let Ok(cgroup_content) = fs::read_to_string("/proc/1/cgroup") {
        if cgroup_content.contains("/docker/") || cgroup_content.contains("/docker-") {
            return true;
        }
        if cgroup_content.contains("/kubepods/") || cgroup_content.contains("/kubepods.slice/") {
            return true;
        }
        if cgroup_content.contains("/lxc/") {
            return true;
        }

        if cgroup_content.contains("/libpod-") {
            return true;
        }
    }

    if let Ok(environ) = fs::read_to_string("/proc/1/environ") {
        if environ.contains("container=") {
            return true;
        }
    }

    if let Ok(mountinfo) = fs::read_to_string("/proc/self/mountinfo") {
        if mountinfo.contains("overlay") && mountinfo.contains("/docker/") {
            return true;
        }
    }

    false
}

fn get_cgroup_path(pid: u64) -> Option<String> {
    let content = fs::read_to_string(format!("/proc/{}/cgroup", pid)).ok()?;
    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 {
            // v2: "0::/path" or v1: find memory/cpu.
            if parts[0] == "0" || parts[1].contains("memory") || parts[1].contains("cpu") {
                return Some(parts[2].to_string());
            }
        }
    }

    None
}

fn main() {
    let env = detect_runtime_environment();
    match &env {
        RuntimeEnvironment::Container => {
            println!("Runtime Environment: Containerized");
        }
        RuntimeEnvironment::Standalone => {
            println!("Runtime Environment: Standalone");
        }
    }

    let pid = std::process::id() as u64;
    let cgroup_path = get_cgroup_path(pid).unwrap_or_else(|| "/".to_string());

    let hier = hierarchies::auto();
    let cg = Cgroup::load(hier, cgroup_path);

    if let Some(mem) = cg.controller_of::<MemController>() {
        let stats = mem.memory_stat();
        println!("Memory Usage: {} bytes", stats.usage_in_bytes);
        println!("Memory Limit: {} bytes", stats.limit_in_bytes);
    }

    if let Some(cpu) = cg.controller_of::<CpuController>() {
        let quota = cpu.cfs_quota().unwrap();
        let period = cpu.cfs_period().unwrap();
        println!("CPU Quota: {} us", quota);
        println!("CPU Period: {} us", period);

        if quota > 0 {
            println!("CPU Limit: {:.2} cores", quota as f64 / period as f64);
        }
    }
}
