use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
/// OTA Server For IoT Devices
pub struct Cli {
    // Firmware Path
    #[clap(long, default_value = "/home/craftor/ftp")]
    pub fw_path: String,

    // Firmware Database
    #[clap(
        long,
        default_value = "postgres://craftor:3.1415926@localhost:50000/firmware"
    )]
    pub fw_db: String,

    /// Listing Port
    #[clap(long, default_value = "20000")]
    pub port: u32,
}
