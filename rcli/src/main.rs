use anyhow::{Ok, Result};
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_generate, process_genpass,
    process_http_serve, process_sign, process_verify, Base64SubCommand, Opts, SubCommand,
    TextSubCommand,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
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
        SubCommand::GenPass(opt) => {
            let pass = process_genpass(
                opt.length,
                opt.uppercase,
                opt.lowercase,
                opt.number,
                opt.symbol,
            )?;
            // make suer the password has at least one of each type
            println!("{}", pass);
            // output password strength in stderr
            let estimate = zxcvbn(&pass, &[]).unwrap();
            eprintln!("Password strength: {}", estimate.score());
            Ok(())
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opt) => {
                let encode = process_encode(&opt.input, opt.format)?;
                println!("{}", encode);
                Ok(())
            }
            Base64SubCommand::Decode(opt) => {
                let decode = process_decode(&opt.input, opt.format)?;
                let decoded = String::from_utf8(decode)?;
                println!("{:?}", decoded);
                Ok(())
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sign = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sign);
                Ok(())
            }
            TextSubCommand::Verify(opts) => {
                let verify = process_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                println!("{}", verify);
                Ok(())
            }
            TextSubCommand::Generate(opts) => {
                let key = process_generate(&opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3-gen.txt");
                        std::fs::write(name, &key[0])?;
                    }
                    rcli::TextSignFormat::Ed25519 => {
                        std::fs::write(opts.output.join("ed25519.sk"), &key[0])?;
                        std::fs::write(opts.output.join("ed25519.pk"), &key[1])?;
                    }
                }
                println!("{:?}", key);
                Ok(())
            }
        },
        SubCommand::Http(subcmd) => match subcmd {
            rcli::HttpSubCommand::Serve(opts) => {
                process_http_serve(&opts.dir, opts.port).await?;
                Ok(())
            }
        },
    }
}
