use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand,
};

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
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opt) => process_encode(&opt.input, opt.format),
            Base64SubCommand::Decode(opt) => process_decode(&opt.input, opt.format),
        },
        SubCommand::Text(subcmd) => match subcmd {
            rcli::TextSubCommand::Sign(opts) => {
                {
                    println!("{:?}", opts);
                };
                Ok(())
            }
            rcli::TextSubCommand::Verify(opts) => {
                {
                    println!("{:?}", opts);
                };
                Ok(())
            }
        },
    }
}
