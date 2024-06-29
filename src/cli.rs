use clap::Parser;

use crate::consts::{CLI_ABOUT, CLI_AUTHOR, CLI_NAME, CLI_VERSION};

#[derive(Parser, Debug)]
#[command(
    name = CLI_NAME,
    version = CLI_VERSION,
    author = CLI_AUTHOR,
    about = CLI_ABOUT
)]
pub struct Args {
    /// Turn on environment flag
    #[arg(short, long)]
    pub env: bool,
}
