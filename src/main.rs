use clap::Parser;
use clap::Subcommand;
use std::io::Write;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug)]
enum Error {
    UNKNOWN,
}

type Result<T> = core::result::Result<T, Error>;

#[derive(Parser, Debug)]
#[command(
    author = "Elias, Karreiro, People watching stream",
    version,
    about = "A cool CLI to track your daily tasks"
)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Create {
        title: String,
    },
    /// List the tasks done at day
    List {
        /// day must be %Y%m%d
        day: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Create { title } => create(title).unwrap(),
        Action::List { day } => list(day).unwrap(),
    }
}

fn list(day: String) -> Result<()> {
    let mut path = get_mtask_path(&day);
    let mut file = std::fs::OpenOptions::new().read(true).open(&path);
    match file {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.unwrap();
                println!("{line}");
            }
        }
        Err(e) => {
            println!("error while reading file {:?}, error: {:?}", path, e);
            exit(-1);
        }
    }
    Ok(())
}

fn create(title: String) -> Result<()> {
    let mut path = get_mtask_path(&get_curr_day());

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("could not create a file for today");

    file.write(title.as_bytes()).unwrap();
    file.write("\n".as_bytes()).unwrap();
    println!("task added");

    Ok(())
}

fn get_mtask_path(day: &str) -> PathBuf {
    let mut path = get_home_dir();
    path.push(".mtasks");
    if !path.exists() {
        std::fs::create_dir(&path).expect("failed to create .mtask folder on $HOME dir");
    }
    path.push(format!("task_{day}"));
    path
}

fn get_curr_day() -> String {
    chrono::offset::Local::now().format("%Y%m%d").to_string()
}

fn get_home_dir() -> PathBuf {
    std::env::var_os("HOME")
        .expect("you need to have $HOME setup")
        .into()
}
