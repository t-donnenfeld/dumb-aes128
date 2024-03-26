use clap::Parser;

#[derive(Parser)]
#[command(name = "dumb-aes128")]
#[command(version = "1.0")]
#[command(about = "Computes aes128 with inline buffer and key", long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    password: String,

    #[arg(short, long, default_value_t = true)]
    encode: bool,
}

fn main() {
    let args = Cli::parse();
    let number_of_rounds = 10;
    let number_of_blocks = 4;
}
