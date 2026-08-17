mod modules;
mod platform;

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

    let system = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    println!("{}", "       /\\_/\\ ".cyan());
    println!("{}", "      ( o.o )".cyan());
    println!("{}", "       > ^ < ".cyan());
    println!();

    println!("OS:       {}", modules::os::get());
    println!("Host:     {}", modules::host::get());
    println!("Kernel:   {}", modules::kernel::get());
    println!("Uptime:   {}", modules::uptime::get());
    println!("CPU:      {}", modules::cpu::get(&system));
    println!("Memory:   {}", modules::memory::get(&system));
    println!("GPU:      {}", modules::gpu::get());
    println!("Disk:     {}", modules::disk::get());
    println!("Shell:    {}", modules::shell::get());
}
