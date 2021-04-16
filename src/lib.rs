use std::ffi::OsString;
use std::path::PathBuf;

pub enum V1controller {
    Cpu,
    Cpuaccount,
    Cpuset,
    Memory,
    Devices,
    Freezer,
    NetCls,
    Blkio,
    PerfEvent,
    NetPrio,
    Hugetlb,
    Pids,
    Rdma,
}

pub struct Cgroup {
    pub filesystem: PathBuf,
}

impl Cgroup {
    pub async fn init() -> Cgroup {
        let os_string = OsString::from("/sys/fs/cgroup/");
        let mut path_buffer = PathBuf::new();

        path_buffer.push(os_string);

        Cgroup {
            filesystem: path_buffer,
        }
    }
}
