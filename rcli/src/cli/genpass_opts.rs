use clap::Parser;
use zxcvbn::zxcvbn;

use crate::CmdExecutor;

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

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let pass = crate::process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        )?;
        // make suer the password has at least one of each type
        println!("{}", pass);
        // output password strength in stderr
        let estimate = zxcvbn(&pass, &[]).unwrap();
        eprintln!("Password strength: {}", estimate.score());
        Ok(())
    }
}
