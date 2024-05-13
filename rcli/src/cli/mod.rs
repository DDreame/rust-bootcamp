mod base_opts;
mod csv_opts;
mod genpass_opts;
mod text;

pub use self::{
    base_opts::{Base64Format, Base64SubCommand},
    csv_opts::OutputFormat,
    text::TextSubCommand,
};

use clap::Parser;
use csv_opts::CsvOpts;
use genpass_opts::GenPassOpts;

#[derive(Debug, Parser)]
#[clap(name="rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show Csv, or convert CSV to Other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(
            verify_input_file("Cargo.toml"),
            Ok("Cargo.toml".to_string())
        );
        assert_eq!(verify_input_file("-"), Ok("-".to_string()));
        assert_eq!(verify_input_file("no-exist"), Err("File does not exist"));
    }
}
