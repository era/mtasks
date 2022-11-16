use clap::Parser;
use clap::Subcommand;
use std::io::Write;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug)]
enum Error {}

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
    /// Adds a task to your today's list
    Create {
        /// The description or title of your task
        title: String,
    },
    /// List the tasks done at day
    List {
        /// lists tasks from day
        /// format must be %Y%m%d
        #[arg(short, long)]
        day: Option<String>,
        /// lists latest file tasks
        #[arg(short, long)]
        last_day: bool,
        /// lists today tasks
        #[arg(short, long)]
        today: bool,
    },
}

enum Day {
    Date(String),
    Today,
    LastDay,
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Create { title } => create(title).unwrap(),
        Action::List {
            day: None,
            last_day: true,
            today: false,
        } => list(Day::LastDay).unwrap(),
        Action::List {
            day: None,
            last_day: false,
            today: true,
        } => list(Day::Today).unwrap(),
        Action::List {
            day: Some(date),
            last_day: false,
            today: false,
        } => list(Day::Date(date)).unwrap(),
        Action::List { .. } => panic!("user is using it wrong"),
    }
}

fn list(day: Day) -> Result<()> {
    let day = match day {
        Day::Date(date) => date,
        Day::Today => get_curr_day(),
        Day::LastDay => todo!(),
    };
    let path = get_mtask_path(&day);
    let file = std::fs::OpenOptions::new().read(true).open(&path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.unwrap();
                println!("{line}");
            }
        }
        Err(e) => {
            println!("Error while trying to open file");
            eprintln!("file: {:?}\nerror: {:?}", path, e);
            exit(-1);
        }
    }
    Ok(())
}

fn create(title: String) -> Result<()> {
    let path = get_mtask_path(&get_curr_day());

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
