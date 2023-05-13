use std::fs::File;
use std::io;
use clap::Parser;
use rand::Rng;
use std::io::{Read, Write};
use std::ops::Add;
use tokio;


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

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("M: {}, N: {}", args.n, args.m);
    if args.read {
        read(args);
    } else {
        write(args).await;
    }
}

fn read(args: Args) {
    for n in 0..args.m {
        let file_name = get_filename(n);
        read_file(file_name);

    }
}

fn read_file(file_name: String) -> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn get_filename(n: u16) -> String {
    n.to_string().add(".data")
}


async fn write(args: Args) {
    let mut tokio_spawns = Vec::new();
    for n in 0..args.m {
        let file_name = get_filename(n);
        tokio_spawns.push(tokio::spawn(write_file(file_name, args.n)));
    }

    futures::future::join_all(tokio_spawns.into_iter()).await;
}

async fn write_file(file_name: String, count: u16) {

    let mut rng = rand::thread_rng();

    let mut file = match File::options()
        .create(true)
        .append(true)
        .open(file_name)
    {
        Ok(file) => {
            file
        }
        Err(_e) => {
            std::process::exit(1)
        }
    };

    for _ in 0..count {
        let data = rng.gen::<u16>();
        let output = serde_json::to_string(&data).unwrap();
        writeln!(&mut file, "{}", output).unwrap();
    }
}