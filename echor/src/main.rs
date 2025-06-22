use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
struct Args {
    #[arg()]
    message: Vec<String>,

    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    print!("{}", args.message.join(" "));
    if !args.omit_newline {
        println!();
    }
}
