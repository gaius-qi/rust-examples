use cgroups_rs::fs::{
    Cgroup, cgroup::get_cgroups_relative_paths_by_pid, cpu::CpuController, hierarchies,
    memory::MemController,
};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeEnvironment {
    Container,
    Systemd,
    Standalone,
}

pub fn detect_runtime_environment() -> RuntimeEnvironment {
    if is_container() {
        return RuntimeEnvironment::Container;
    }

    if is_systemd() {
        return RuntimeEnvironment::Systemd;
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

fn is_systemd() -> bool {
    if std::env::var("INVOCATION_ID").is_ok() {
        return true;
    }

    if std::env::var("JOURNAL_STREAM").is_ok() {
        return true;
    }

    if let Ok(cgroup_content) = fs::read_to_string("/proc/self/cgroup") {
        if cgroup_content.contains(".service") || cgroup_content.contains(".scope") {
            return true;
        }
    }

    if let Ok(ppid_cmdline) = fs::read_to_string("/proc/self/stat") {
        let parts: Vec<&str> = ppid_cmdline.split_whitespace().collect();
        if parts.len() > 3 {
            if let Ok(ppid) = parts[3].parse::<u32>() {
                if ppid == 1 {
                    if let Ok(init_comm) = fs::read_to_string("/proc/1/comm") {
                        if init_comm.trim() == "systemd" {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

fn main() {
    let env = detect_runtime_environment();
    match &env {
        RuntimeEnvironment::Container => {
            println!("Runtime Environment: Containerized");

            let pid = std::process::id();
            let relative_paths = get_cgroups_relative_paths_by_pid(pid).unwrap();
            println!("Relative Path: {:?}", relative_paths);

            let hier = hierarchies::auto();
            println!("Cgroup Hierarchy: {}", hier.v2());

            let cg = Cgroup::load_with_relative_paths(hier, "", relative_paths);
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
        RuntimeEnvironment::Systemd => {
            println!("Runtime Environment: Systemd");
        }
        RuntimeEnvironment::Standalone => {
            println!("Runtime Environment: Standalone");
        }
    }
}
