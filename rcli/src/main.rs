use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opt) => {
            let output = if let Some(output) = opt.output {
                output.clone()
            } else {
                format!("output.{}", opt.format)
            };
            process_csv(&opt.input, &output, opt.format)
        }
        SubCommand::GenPass(opt) => process_genpass(
            opt.length,
            opt.uppercase,
            opt.lowercase,
            opt.number,
            opt.symbol,
        ),
    }
}
