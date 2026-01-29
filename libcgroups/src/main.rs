use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeEnvironment {
    Container,
    Systemd,
    Standalone,
}

pub fn detect_runtime_environment() -> RuntimeEnvironment {
    // 1. 首先检测是否在容器中
    if is_container() {
        return RuntimeEnvironment::Container;
    }

    // 2. 检测是否由 systemd 管理
    if is_systemd() {
        return RuntimeEnvironment::Systemd;
    }

    // 3. 否则是直接运行的进程
    RuntimeEnvironment::Standalone
}

fn is_container() -> bool {
    // 方法1: 检查 /.dockerenv 文件 (Docker)
    if Path::new("/.dockerenv").exists() {
        return true;
    }

    // 方法2: 检查 /run/.containerenv (Podman)
    if Path::new("/run/.containerenv").exists() {
        return true;
    }

    // 方法3: 检查 Kubernetes 环境变量
    if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
        return true;
    }

    // 方法4: 检查 /proc/1/cgroup 文件内容
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
        // cgroup v2 的情况
        if cgroup_content.contains("/libpod-") {
            return true;
        }
    }

    // 方法5: 检查 /proc/1/environ 中的容器相关环境变量
    if let Ok(environ) = fs::read_to_string("/proc/1/environ") {
        if environ.contains("container=") {
            return true;
        }
    }

    // 方法6: 检查 /proc/self/mountinfo 中的 overlay 文件系统
    if let Ok(mountinfo) = fs::read_to_string("/proc/self/mountinfo") {
        if mountinfo.contains("overlay") && mountinfo.contains("/docker/") {
            return true;
        }
    }

    false
}

/// 检测是否由 systemd 管理
fn is_systemd() -> bool {
    // 方法1: 检查 INVOCATION_ID 环境变量 (systemd 设置)
    if std::env::var("INVOCATION_ID").is_ok() {
        return true;
    }

    // 方法2: 检查 JOURNAL_STREAM 环境变量
    if std::env::var("JOURNAL_STREAM").is_ok() {
        return true;
    }

    // 方法3: 检查当前进程的 cgroup 路径是否包含 .service
    if let Ok(cgroup_content) = fs::read_to_string("/proc/self/cgroup") {
        if cgroup_content.contains(".service") || cgroup_content.contains(".scope") {
            return true;
        }
    }

    // 方法4: 检查父进程是否是 systemd
    if let Ok(ppid_cmdline) = fs::read_to_string("/proc/self/stat") {
        let parts: Vec<&str> = ppid_cmdline.split_whitespace().collect();
        if parts.len() > 3 {
            if let Ok(ppid) = parts[3].parse::<u32>() {
                if ppid == 1 {
                    // 父进程是 PID 1，检查是否是 systemd
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

use libcgroups::common::{CgroupManager, CgroupSetup, ControllerOpt};
use libcgroups::stats::Stats;

/// 资源限制和使用情况
#[derive(Debug, Default)]
pub struct ResourceInfo {
    /// 内存限制 (bytes)
    pub memory_limit: Option<u64>,
    /// 内存使用量 (bytes)
    pub memory_usage: Option<u64>,
    /// 内存使用百分比
    pub memory_usage_percent: Option<f64>,
    /// CPU 限制 (quota/period, 表示可用 CPU 核心数)
    pub cpu_limit: Option<f64>,
    /// CPU 使用时间 (nanoseconds)
    pub cpu_usage_ns: Option<u64>,
    /// CPU 周期数
    pub cpu_periods: Option<u64>,
    /// CPU 被限制的周期数
    pub cpu_throttled_periods: Option<u64>,
}

/// 使用 libcgroups 获取资源信息
pub fn get_resource_info_with_libcgroups() -> Result<ResourceInfo, Box<dyn std::error::Error>> {
    use libcgroups::common::create_cgroup_manager;
    use std::path::PathBuf;

    let mut resource_info = ResourceInfo::default();

    // 检测 cgroup 版本并获取根路径
    let cgroup_setup = libcgroups::common::get_cgroup_setup().inspect_err(|err| {
        eprintln!("检测 cgroup 设置失败: {}", err);
    })?;

    let (cgroup_root, use_systemd) = match cgroup_setup {
        CgroupSetup::Legacy => ("/sys/fs/cgroup".into(), false),
        CgroupSetup::Unified => ("/sys/fs/cgroup".into(), false),
        CgroupSetup::Hybrid => ("/sys/fs/cgroup/unified".into(), false),
    };

    // 创建 cgroup manager
    // create_cgroup_manager 的参数根据版本可能不同，这里展示通用方式
    let manager = create_cgroup_manager(cgroup_root).inspect_err(|err| {
        eprintln!("创建 cgroup 管理器失败: {}", err);
    })?;

    // 获取统计信息
    let stats = manager.stats().inspect_err(|err| {
        eprintln!("获取 cgroup 统计信息失败: {}", err);
    })?;

    // 提取内存信息
    if let Some(memory_stats) = &stats.memory {
        resource_info.memory_usage = Some(memory_stats.usage.usage);
        resource_info.memory_limit =
            if memory_stats.usage.limit > 0 && memory_stats.usage.limit < u64::MAX / 2 {
                Some(memory_stats.usage.limit)
            } else {
                None
            };

        // 计算使用百分比
        if let (Some(usage), Some(limit)) = (resource_info.memory_usage, resource_info.memory_limit)
        {
            if limit > 0 {
                resource_info.memory_usage_percent = Some((usage as f64 / limit as f64) * 100.0);
            }
        }
    }

    // 提取 CPU 信息
    if let Some(cpu_stats) = &stats.cpu {
        resource_info.cpu_usage_ns = Some(cpu_stats.usage.usage_total);

        // CPU throttling 信息
        resource_info.cpu_periods = Some(cpu_stats.throttling.periods);
        resource_info.cpu_throttled_periods = Some(cpu_stats.throttling.throttled_periods);
    }

    Ok(resource_info)
}

// ============ 主函数示例 ============

fn main() {
    println!("=== 进程运行环境检测 ===\n");

    // 1. 检测运行环境
    let env = detect_runtime_environment();
    match &env {
        RuntimeEnvironment::Container => {
            println!("运行环境: 容器");
        }
        RuntimeEnvironment::Systemd => {
            println!("运行环境: systemd 管理的服务");
        }
        RuntimeEnvironment::Standalone => {
            println!("运行环境: 独立进程");
        }
    }

    // 3. 使用 libcgroups (需要添加依赖)
    println!("\n=== 资源信息 (libcgroups) ===\n");
    match get_resource_info_with_libcgroups() {
        Ok(info) => {
            println!("内存限制: {:?}", info.memory_limit);
            println!("内存使用: {:?}", info.memory_usage);
            println!("CPU 限制: {:?}", info.cpu_limit);
            println!("CPU 使用(ns): {:?}", info.cpu_usage_ns);
        }
        Err(e) => {
            println!("获取资源信息失败: {}", e);
        }
    }
}
