// src/modules/disk.rs
use sysinfo::Disks;

pub fn get() -> String {
    let disks = Disks::new_with_refreshed_list();

    let root = disks
        .iter()
        .find(|disk| disk.mount_point().to_string_lossy() == "/");

    match root {
        Some(disk) => {
            let total = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let available = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let used = total - available;

            format!("{used:.2} GiB / {total:.2} GiB")
        }
        None => "Unknown".to_string(),
    }
}
