use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
/// OTA Server For IoT Devices
pub struct Cli {
    // Firmware Path
    #[clap(long, default_value = "/home/craftor/ftp")]
    pub fw_path: String,

    /// Listing Port
    #[clap(long, default_value = "20000")]
    pub port: u32,
}
