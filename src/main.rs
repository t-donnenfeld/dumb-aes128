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
    key_expansion();
}

fn cypher() {}

fn inv_cypher() {}

fn key_expansion() {
    let r_con = get_r_con();
    for ele in r_con {
        println!("{:?}", ele);
    }
}

fn get_r_con() -> std::vec::Vec<[i32; 4]> {
    let mut r_con: Vec<_> = (1..8)
        .map(|x| i32::pow(2, x))
        .map(|x| [x, 0, 0, 0])
        .collect();
    r_con.push([27, 0, 0, 0]);
    r_con.push([54, 0, 0, 0]);
    r_con
}
