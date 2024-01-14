use clap::Parser;

#[derive(Parser, PartialEq, Clone)]
#[clap(author, version, about)]
/// OTA Server For IoT Devices
pub struct Cli {
    /// Firmware File Server
    #[clap(long, default_value = "http://127.0.0.1:20000")]
    pub fw_server: String,

    /// API Listening Port
    #[clap(long, default_value = "9999")]
    pub port: u32,
}
