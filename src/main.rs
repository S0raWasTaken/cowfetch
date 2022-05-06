use colors::*;
use columns::Columns;
use sys_info::{cpu_num, disk_info, hostname, linux_os_release, mem_info, os_release};

mod colors;

const COW: &str = r"
|
|
| \   ^__^
|  \  (oo)\_______
|     (__)\       )\/\
|         ||----w |
|         ||     ||
";
const DEFAULT: &str = "Unknown";
fn main() {
    let os = linux_os_release().unwrap_or_default();
    println!(
        "{}",
        Columns::from(vec![
            vec![
                &*format!("{BLUE} -----------------------------"),
                &*format!("{BLUE}| {RED}OS{RESET}: {}{BLUE}", os.pretty_name()),
                &*format!(
                    "{BLUE}| {BLUE}Version{RESET}: {}{BLUE}",
                    os.build_id.unwrap_or_else(|| String::from(DEFAULT))
                ),
                &*format!(
                    "{BLUE}| {YELLOW}Hostname{RESET}: {}{BLUE}",
                    hostname().unwrap_or_else(|_| String::from(DEFAULT))
                ),
                &*format!(
                    "{BLUE}| {PURPLE}Kernel{RESET}: {}{BLUE}",
                    os_release().unwrap_or_else(|_| String::from(DEFAULT))
                ),
                &*format!(
                    "{BLUE}| {CYAN}CPU_Threads{RESET}: {}{BLUE}",
                    cpu_num().unwrap_or(0)
                ),
                &*format!(
                    "{BLUE}| {GREEN}Mem_total{RESET}: {} GiB{BLUE}",
                    mem_info().unwrap().total / 1024000
                ),
                &*format!(
                    "{BLUE}| {GREEN}Disk_total{RESET}: {} GiB{BLUE}",
                    disk_info().unwrap().total / 1024000,
                ),
                &*format!("{BLUE} -----------------------------"),
            ],
            COW.split('\n').collect::<Vec<_>>(),
        ])
    );
}
