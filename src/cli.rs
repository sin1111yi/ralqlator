use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "raculator")]
#[command(about = "A command line calculator", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Expression to calculate
    #[arg()]
    pub expression: Option<String>,

    /// Expression to calculate (shorthand)
    #[arg(short = 'r', long = "row")]
    pub row: Option<String>,

    /// Output result in hexadecimal format
    #[arg(short = 'x', long = "hex")]
    pub hex: bool,

    /// Output result in octal format
    #[arg(short = 'o', long = "oct")]
    pub oct: bool,

    /// Output result in binary format
    #[arg(short = 'b', long = "bin")]
    pub bin: bool,

    /// Bitwise operation mode (use &, |, ^, ~, <<, >>)
    #[arg(short = 'B', long = "bits")]
    pub bits: bool,

    /// Show supported functions
    #[arg(short = 'F', long = "functions", action = clap::ArgAction::SetTrue)]
    pub show_functions: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show supported functions
    Functions,
}
