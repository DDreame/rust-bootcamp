use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name="rcli", version, author, about, long_about = None)]
pub struct  Opts{
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show Csv, or convert CSV to Other formats")]
    Csv(CsvOpts)
}

#[derive(Debug, Parser)]
pub struct CsvOpts{
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = 't')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool

}

fn verify_input_file(filename: &str) -> Result<String, &'static str>{
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}