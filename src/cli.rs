use clap::Parser;

/// Backend for https://www.mir4scope.com
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Initial page to collect NFT
    #[arg(short, long, default_value_t = 0)]
    pub initial_page: u8,

    /// Final page to collect NFT
    #[arg(short, long, default_value_t = 0)]
    pub final_page: u8,

    /// If the backend should drop the database or not.
    #[arg(short, long, default_value_t = true)]
    pub drop: bool,
}
