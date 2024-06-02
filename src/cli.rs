use clap::Parser;

/// Backend for https://www.mir4scope.com
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Initial page to collect NFT
    #[arg(short, long, default_value_t = 1)]
    pub initial_page: u8,

    /// Final page to collect NFT
    #[arg(short, long, default_value_t = 5)]
    pub final_page: u8,

    /// If the backend should drop the database or not. [Default: false]
    #[arg(short, long, default_value_t = false)]
    pub drop: bool,

    /// Local Development [default: false]
    #[arg(short, long, default_value_t = false)]
    pub local: bool,
}
