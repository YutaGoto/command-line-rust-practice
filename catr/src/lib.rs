use clap::Parser;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(
    name = "catr",
    version = "1.0",
    about = "A simple file concatenation tool"
)]
pub struct Config {
    #[arg(required = true, help = "Files to concatenate")]
    files: Vec<String>,

    #[arg(short = 'n', long, help = "Number lines in output")]
    number_lines: bool,

    #[arg(short = 'b', long, help = "Number non-blank lines in output")]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let args = Config::parse();

    if args.files.is_empty() {
        eprintln!("Error: No files provided.");
        std::process::exit(1);
    }

    Ok(args)
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    println!("Hello, world!");
    Ok(())
}
