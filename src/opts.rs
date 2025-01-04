use std::path::Path;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about=None)]
pub struct Opts {
    // Example of Jira number: "GDTA-1234"
    #[arg(value_parser = verify_jira_number)]
    pub jira_number: String,

    #[command(subcommand)]
    pub cmd: Subcommand,

}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_jira_number(jira_number: &str) -> Result<String, &'static str> {
    if jira_number.starts_with("GDTA-") {
        Ok(jira_number.into())
    } else {
        Err("Jira number must start with 'GDTA-'")
    }
}
