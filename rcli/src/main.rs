use rcli::{process_csv, Opts, SubCommand};
use clap::Parser;


fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opt) => process_csv(&opt.input, &opt.output),
        
    }
    
}