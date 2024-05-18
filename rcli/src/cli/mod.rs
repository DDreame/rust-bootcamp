mod base_opts;
mod csv_opts;
mod genpass_opts;
mod http;
mod text;

use std::path::{Path, PathBuf};

pub use self::{base_opts::*, csv_opts::*, genpass_opts::GenPassOpts, http::*, text::*};

use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[clap(name="rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show Csv, or convert CSV to Other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Sign/Verify text")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

// impl CmdExecutor for SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             SubCommand::Csv(opt) => opt.execute().await,
//             SubCommand::GenPass(opt) => opt.execute().await,
//             SubCommand::Base64(subcmd) => subcmd.execute().await,
//             SubCommand::Text(subcmd) => subcmd.execute().await,
//             SubCommand::Http(subcmd) => subcmd.execute().await,
//         }
//     }
// }

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        Err("Path does not exist or is not a dir")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(verify_file("no-exist"), Err("File does not exist"));
    }
}
