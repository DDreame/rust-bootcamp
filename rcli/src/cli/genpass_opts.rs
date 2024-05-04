use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value = "16")]
    pub length: u8,

    #[arg(long, default_value = "false")]
    pub uppercase: bool,

    #[arg(long, default_value = "false")]
    pub lowercase: bool,

    #[arg(long, default_value = "false")]
    pub number: bool,

    #[arg(long, default_value = "false")]
    pub symbol: bool,
}
