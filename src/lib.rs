use std::ffi::OsString;
use std::path::PathBuf;

use tokio::fs::read_dir;
use tokio::fs::read_to_string;
// use tokio::fs::File;
// use tokio::io::AsyncReadExt;

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

    pub async fn collect_controller(&mut self, controller: V1controller) {
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

        let path = self.filesystem.join(os_string);
        let read_dir = read_dir(&path).await;

        match read_dir {
            Ok(mut contents) => {
                while let Ok(entries) = contents.next_entry().await {
                    if let Some(entry) = entries {
                        println!("File name - {:?}", entry.file_name());
                        println!("Path - {:?}", entry.path());
                        let contents = read_to_string(entry.path().as_path()).await;
                        match contents {
                            Ok(content) => println!("{}", content),
                            Err(error) => println!("{:?}", error),
                        }
                    } else {
                        break;
                    }
                }
            }
            Err(error) => println!("{:?}", error),
        }
    }
}

pub struct Subsystem {
    pub state: Vec<Vec<u8>>,
}

impl Subsystem {
    pub async fn init() -> Subsystem {
        Subsystem {
            state: Vec::with_capacity(50),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn init() {
        let cgroup = Cgroup::init().await;
        assert_eq!(cgroup.filesystem.is_dir(), true);
    }

    #[tokio::test]
    async fn collect_cpu() {
        let mut test_cgroup = Cgroup::init().await;
        let test_cpu_controller = V1controller::Cpu;
        test_cgroup.collect_controller(test_cpu_controller).await;
    }

    #[tokio::test]
    async fn collect_cpu_accounting() {
        let mut test_cgroup = Cgroup::init().await;
        let test_cpu_accounting_controller = V1controller::Cpuacct;
        test_cgroup
            .collect_controller(test_cpu_accounting_controller)
            .await;
    }

    #[tokio::test]
    async fn collect_cpu_set() {
        let mut test_cgroup = Cgroup::init().await;
        let test_cpu_set_controller = V1controller::Cpuset;
        test_cgroup
            .collect_controller(test_cpu_set_controller)
            .await;
    }

    #[tokio::test]
    async fn collect_memory() {
        let mut test_cgroup = Cgroup::init().await;
        let test_memory_controller = V1controller::Memory;
        test_cgroup.collect_controller(test_memory_controller).await;
    }

    #[tokio::test]
    async fn collect_devices() {
        let mut test_cgroup = Cgroup::init().await;
        let test_devices_controller = V1controller::Devices;
        test_cgroup
            .collect_controller(test_devices_controller)
            .await;
    }

    #[tokio::test]
    async fn collect_freezer() {
        let mut test_cgroup = Cgroup::init().await;
        let test_freezer_controller = V1controller::Freezer;
        test_cgroup
            .collect_controller(test_freezer_controller)
            .await;
    }

    #[tokio::test]
    async fn collect_netcls() {
        let mut test_cgroup = Cgroup::init().await;
        let test_netcls_controller = V1controller::NetCls;
        test_cgroup.collect_controller(test_netcls_controller).await;
    }

    #[tokio::test]
    async fn collect_blkio() {
        let mut test_cgroup = Cgroup::init().await;
        let test_blkio_controller = V1controller::Blkio;
        test_cgroup.collect_controller(test_blkio_controller).await;
    }

    #[tokio::test]
    async fn collect_perfevent() {
        let mut test_cgroup = Cgroup::init().await;
        let test_perfevent_controller = V1controller::PerfEvent;
        test_cgroup
            .collect_controller(test_perfevent_controller)
            .await;
    }

    #[tokio::test]
    async fn collect_netprio() {
        let mut test_cgroup = Cgroup::init().await;
        let test_netprio_controller = V1controller::NetPrio;
        test_cgroup
            .collect_controller(test_netprio_controller)
            .await;
    }

    #[tokio::test]
    async fn collect_hugetlb() {
        let mut test_cgroup = Cgroup::init().await;
        let test_hugetlb_controller = V1controller::Hugetlb;
        test_cgroup
            .collect_controller(test_hugetlb_controller)
            .await;
    }

    #[tokio::test]
    async fn collect_pids() {
        let mut test_cgroup = Cgroup::init().await;
        let test_pids_controller = V1controller::Pids;
        test_cgroup.collect_controller(test_pids_controller).await;
    }

    #[tokio::test]
    async fn collect_rdma() {
        let mut test_cgroup = Cgroup::init().await;
        let test_rdma_controller = V1controller::Rdma;
        test_cgroup.collect_controller(test_rdma_controller).await;
    }
}
