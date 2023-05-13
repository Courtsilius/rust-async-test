use std::fs::File;
use clap::Parser;
use rand::Rng;
use std::io::Write;
use std::ops::Add;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    n: u16,

    #[arg(short, long)]
    m: u16,

    #[clap(long, short, action)]
    read: bool,

    #[clap(long, short, action)]
    write: bool,
}

fn main() {
    let args = Args::parse();

    println!("M: {}, N: {}", args.n, args.m);
    if args.read {
        read(args);
    } else {
        write(args);
    }
}

fn read(args: Args) {
    todo!();
}

fn write(args: Args) {
    for n in 0..args.n {
        let mut file_name = n.to_string().add(".txt");
        write_file(&file_name, args.m)
    }
}

fn write_file(file_name: &String, count: u16) {

    let mut rng = rand::thread_rng();

    let mut file = match File::options()
        .create(true)
        .append(true)
        .open(file_name)
    {
        Ok(file) => {
            file
        }
        Err(e) => {
            std::process::exit(1)
        }
    };

    for _ in 0..count {
        let data = rng.gen::<u16>();
        let output = serde_json::to_string(&data).unwrap();
        writeln!(&mut file, "{}", output).unwrap();
    }
}