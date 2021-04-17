use std::ffi::OsString;
use std::path::PathBuf;

use tokio::fs::read_dir;
// use tokio::fs::File;
// use tokio::io::AsyncReadExt;

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

    pub async fn collect_cpu(&mut self) {
        let cpu_os_string = OsString::from("cpu/");
        let cpu_path = self.filesystem.join(cpu_os_string);
        let read_dir = read_dir(&cpu_path).await;

        match read_dir {
            Ok(mut contents) => {
                while let Ok(entries) = contents.next_entry().await {
                    if let Some(entry) = entries {
                        println!("File name - {:?}", entry.file_name());
                        println!("Path - {:?}", entry.path());
                    } else {
                        break;
                    }
                }
            }
            Err(error) => println!("{:?}", error),
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
        let mut cgroup = Cgroup::init().await;
        cgroup.collect_cpu().await;
    }
}
