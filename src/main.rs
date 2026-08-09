use clap::Parser;
use owo_colors::OwoColorize;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

#[derive(Parser, Debug)]
#[command(
    name = "orphfetch",
    version,
    about = "A fast system information tool written in Rust"
)]
struct Args {}

fn main() {
    let _args = Args::parse();

    let mut system = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    system.refresh_all();

    let os = System::long_os_version().unwrap_or_else(|| "Unknown".into());
    let host = System::host_name().unwrap_or_else(|| "Unknown".into());
    let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".into());
    let uptime = System::uptime();

    let cpu = system
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown".into());

    let memory_used = system.used_memory() / 1024 / 1024 / 1024;
    let memory_total = system.total_memory() / 1024 / 1024 / 1024;

    println!("{}", "       /\\_/\\ ".cyan());
    println!("{}", "      ( o.o )".cyan());
    println!("{}", "       > ^ < ".cyan());
    println!();

    println!("{} {}", "OS:".bold(), os);
    println!("{} {}", "Host:".bold(), host);
    println!("{} {}", "Kernel:".bold(), kernel);
    println!("{} {} seconds", "Uptime:".bold(), uptime);
    println!("{} {}", "CPU:".bold(), cpu);
    println!(
        "{} {} GiB / {} GiB",
        "Memory:".bold(),
        memory_used,
        memory_total
    );
}
