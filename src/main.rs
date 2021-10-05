use colorful::Color;
use colorful::Colorful;
use sys_info::*;

fn main() -> Result<(), String> {
    let cpu_threads = cpu_num().map_err(|_| String::from("No threads"))?;

    let diskinfo = disk_info().map_err(|_| String::from("No disk info"))?;
    let hostname = hostname().map_err(|_| String::from("No hostname"))?;

    let kernel_info = os_release().map_err(|_| String::from("No kernel info"))?;

    let release_info = linux_os_release().map_err(|_| String::from("No release info"))?;
    let pretty_distro_name = release_info
        .pretty_name
        .ok_or(1)
        .map_err(|_| String::from("No distro name"))?;
    let version = release_info
        .build_id
        .ok_or(2)
        .map_err(|_| String::from("No release version"))?;

    let meminfo = mem_info().map_err(|_| String::from("No memory info"))?;

    println!(
        "   {}    {}: {}\n    {}: {}\n    {}: {}\n    {}: {}\n    {}: {}\n    {}: {} GiB\n    {}: {} MiB\n  {}",
        "___________________________\n  /                           \\\n".color(Color::MediumPurple),
        "OS".color(Color::Red),
        pretty_distro_name,
        "Version".color(Color::LightBlue),
        version,
        "Hostname".color(Color::LightCoral),
        hostname,
        "Kernel".color(Color::Yellow),
        kernel_info,
        "CPU_Threads".color(Color::Pink1),
        cpu_threads,
        "Disk_Total".color(Color::Green),
        diskinfo.total/1024000,
        "Mem_Total".color(Color::Orange1),
        meminfo.total/1024,
        "\\___________________________/".color(Color::MediumPurple)
    );

    let cowsay = "          \\
           \\   ^__^
            \\  (oo)\\_______
               (__)\\       )\\/\\
                   ||----w |
                   ||     ||";
    
    println!("{}", cowsay.color(Color::MediumPurple));

    Ok(())
}
