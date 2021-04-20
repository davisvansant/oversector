use std::ffi::OsString;
use std::path::PathBuf;

use tokio::fs::read;
use tokio::fs::read_dir;

pub enum V1controller {
    Cpu,
    Cpuacct,
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

pub struct Subsystem {
    pub state: Vec<Vec<Vec<u8>>>,
    hierarchy: PathBuf,
}

impl Subsystem {
    pub async fn init(cgroup: &Cgroup) -> Subsystem {
        Subsystem {
            state: Vec::with_capacity(50),
            hierarchy: cgroup.filesystem.to_path_buf(),
        }
    }

    pub async fn collect(&mut self, controller: &V1controller) {
        let mut vec: Vec<Vec<u8>> = Vec::with_capacity(10);

        let os_string = match controller {
            V1controller::Cpu => OsString::from("cpu/"),
            V1controller::Cpuacct => OsString::from("cpuacct/"),
            V1controller::Cpuset => OsString::from("cpuset/"),
            V1controller::Memory => OsString::from("memory/"),
            V1controller::Devices => OsString::from("devices/"),
            V1controller::Freezer => OsString::from("freezer/"),
            V1controller::NetCls => OsString::from("net_cls/"),
            V1controller::Blkio => OsString::from("blkio/"),
            V1controller::PerfEvent => OsString::from("perf_event/"),
            V1controller::NetPrio => OsString::from("net_prio/"),
            V1controller::Hugetlb => OsString::from("hugetlb/"),
            V1controller::Pids => OsString::from("pids/"),
            V1controller::Rdma => OsString::from("rdma/"),
        };

        let path = self.hierarchy.join(os_string);
        let read_dir = read_dir(&path).await;

        match read_dir {
            Ok(mut contents) => {
                while let Ok(entries) = contents.next_entry().await {
                    if let Some(entry) = entries {
                        let contents = read(entry.path().as_path()).await;
                        match contents {
                            Ok(content) => {
                                vec.push(content);
                            }
                            Err(error) => println!("{:?}", error),
                        }
                    } else {
                        break;
                    }
                }
            }
            Err(error) => println!("{:?}", error),
        }
        self.state.push(vec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cgroup_init() {
        let cgroup = Cgroup::init().await;
        assert_eq!(cgroup.filesystem.is_dir(), true);
    }

    #[tokio::test]
    async fn subsystem_init() {
        let test_cgroup = Cgroup::init().await;
        let test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
    }

    #[tokio::test]
    async fn collect_cpu() {
        let test_cgroup = Cgroup::init().await;
        let test_cpu_controller = V1controller::Cpu;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_cpu_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_cpu_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_cpu_accounting() {
        let test_cgroup = Cgroup::init().await;
        let test_cpu_accounting_controller = V1controller::Cpuacct;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem
            .collect(&test_cpu_accounting_controller)
            .await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem
            .collect(&test_cpu_accounting_controller)
            .await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_cpu_set() {
        let test_cgroup = Cgroup::init().await;
        let test_cpu_set_controller = V1controller::Cpuset;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_cpu_set_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_cpu_set_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_memory() {
        let test_cgroup = Cgroup::init().await;
        let test_memory_controller = V1controller::Memory;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_memory_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_memory_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_devices() {
        let test_cgroup = Cgroup::init().await;
        let test_devices_controller = V1controller::Devices;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_devices_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_devices_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_freezer() {
        let test_cgroup = Cgroup::init().await;
        let test_freezer_controller = V1controller::Freezer;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_freezer_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_freezer_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_netcls() {
        let test_cgroup = Cgroup::init().await;
        let test_netcls_controller = V1controller::NetCls;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_netcls_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_netcls_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_blkio() {
        let test_cgroup = Cgroup::init().await;
        let test_blkio_controller = V1controller::Blkio;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_blkio_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_blkio_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_perfevent() {
        let test_cgroup = Cgroup::init().await;
        let test_perfevent_controller = V1controller::PerfEvent;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_perfevent_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_perfevent_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_netprio() {
        let test_cgroup = Cgroup::init().await;
        let test_netprio_controller = V1controller::NetPrio;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_netprio_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_netprio_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_hugetlb() {
        let test_cgroup = Cgroup::init().await;
        let test_hugetlb_controller = V1controller::Hugetlb;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_hugetlb_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_hugetlb_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_pids() {
        let test_cgroup = Cgroup::init().await;
        let test_pids_controller = V1controller::Pids;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_pids_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_pids_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }

    #[tokio::test]
    async fn collect_rdma() {
        let test_cgroup = Cgroup::init().await;
        let test_rdma_controller = V1controller::Rdma;
        let mut test_subsystem = Subsystem::init(&test_cgroup).await;
        assert_eq!(test_subsystem.state.len(), 0);
        assert_eq!(test_subsystem.hierarchy, PathBuf::from("/sys/fs/cgroup/"));
        test_subsystem.collect(&test_rdma_controller).await;
        assert_eq!(test_subsystem.state.len(), 1);
        test_subsystem.collect(&test_rdma_controller).await;
        assert_eq!(test_subsystem.state.len(), 2);
    }
}
