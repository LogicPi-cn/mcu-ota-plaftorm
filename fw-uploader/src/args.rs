use clap::{Parser, Subcommand};

/// Firmware Uploader
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "fwupload")]
#[command(about = "A firmware uploader for firmware.", long_about = None)]
pub struct Cli {
    // Firmware Server
    #[clap(long, default_value = "http://127.0.0.1:20000/firmware")]
    pub server: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    List {
        // Firmware Code
        #[clap(long)]
        fw_code: String,
    },

    #[command(arg_required_else_help = true)]
    Upload {
        // Firmware Code
        #[clap(long)]
        fw_code: String,

        // Firmware Version
        #[clap(long)]
        fw_version: String,

        // Firmware Path
        #[clap(long)]
        fw_path: String,
    },
}
