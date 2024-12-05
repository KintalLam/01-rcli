// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
/* use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: Subcommand,
}

#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts)
}

fn verify_input_file
*/
