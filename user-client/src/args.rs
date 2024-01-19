use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
/// OTA Server For IoT Devices
pub struct Cli {
    // Firmware Server
    #[clap(long)]
    pub server: String,

    // Firmware Code
    #[clap(long)]
    pub fw_code: i32,

    // Firmware Version
    #[clap(long)]
    pub fw_version: String,

    // Firmware Path
    #[clap(long)]
    pub fw_path: String,
}
