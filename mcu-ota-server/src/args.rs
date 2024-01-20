use clap::Parser;

#[derive(Parser, PartialEq, Clone)]
#[clap(author, version, about)]
/// OTA Server For IoT Devices
pub struct Cli {
    /// Firmware File Server
    #[clap(long, default_value = "http://127.0.0.1:20000")]
    pub fw_server: String,

    // Firmware Database
    #[clap(
        long,
        default_value = "postgres://craftor:3.1415926@localhost:5432/firmware"
    )]
    pub fw_db: String,

    /// API Listening Port
    #[clap(long, default_value = "9999")]
    pub port: u32,
}
