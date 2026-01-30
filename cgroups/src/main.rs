use cgroups_rs::fs::{
    Cgroup, cgroup::get_cgroups_relative_paths_by_pid, cpu::CpuController, hierarchies,
    memory::MemController,
};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

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
        }
        RuntimeEnvironment::Systemd => {
            println!("Runtime Environment: Systemd");
        }
        RuntimeEnvironment::Standalone => {
            println!("Runtime Environment: Standalone");
        }
    }

    let pid = std::process::id();
    let hierarchies = hierarchies::auto();
    let cg = if hierarchies.v2() {
        let path = get_cgroups_v2_path_by_pid(pid);
        Cgroup::load(hierarchies, path)
    } else {
        // get container main process cgroup
        let path = get_cgroups_relative_paths_by_pid(pid).unwrap();
        Cgroup::load_with_relative_paths(hierarchies::auto(), Path::new("."), path)
    };

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

pub fn get_cgroups_v2_path_by_pid(pid: u32) -> PathBuf {
    let content = fs::read_to_string(format!("/proc/{}/cgroup", pid)).unwrap();
    let relative_path = content
        .lines()
        .next()
        .unwrap()
        .strip_prefix("0::")
        .unwrap()
        .trim_start_matches('/');

    PathBuf::from("/sys/fs/cgroup").join(relative_path)
}

// Get the cgroups v2 path given a PID
// pub fn get_cgroups_v2_path_by_pid(pid: u32) -> PathBuf {
// let path = format!("/proc/{}/cgroup", pid);
// let content = fs::read_to_string(path).unwrap();
// let content = content.lines().next().unwrap_or("");
// parse_cgroups_v2_path(content).canonicalize().unwrap()
// }

// https://github.com/opencontainers/runc/blob/1950892f69597aa844cbf000fbdf77610dda3a44/libcontainer/cgroups/fs2/defaultpath.go#L83
// fn parse_cgroups_v2_path(content: &str) -> PathBuf {
// the entry for cgroup v2 is always in the format like `0::$PATH`
// where 0 is the hierarchy ID, the controller name is omitted in cgroup v2
// and $PATH is the cgroup path
// see https://docs.kernel.org/admin-guide/cgroup-v2.html
// let path = content.strip_prefix("0::").unwrap();
// let path = path.trim_start_matches('/');

// PathBuf::from(format!("/sys/fs/cgroup/{}", path))
// }
