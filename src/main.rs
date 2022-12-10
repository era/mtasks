use clap::Parser;
use clap::Subcommand;
use regex::Regex;
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
        /// day that you want to add if not set, today is picked
        #[arg(short, long)]
        day: Option<String>,
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
        Action::Create { title, day: None } => create(title, get_curr_day()).unwrap(),
        Action::Create {
            title,
            day: Some(day),
        } => create(title, day).unwrap(),
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
        Day::LastDay => get_last_day(),
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

fn get_last_day() -> String {
    // if there is no "last day" we panic with the user error message
    // we list the files in the folder, sort and get the last that is not today
    let files = std::fs::read_dir(get_mtask_dir()).unwrap();
    let curr_day = get_curr_day();
    let mut files: Vec<String> = files
        .map(|file| {
            file.unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .replace("task_", "")
        })
        .filter(|file_name| file_name != &curr_day)
        .collect();

    files.sort();

    match files.last() {
        Some(e) => e.to_owned(),
        None => panic!("You have not done anything in the past (no tasks added to mtask)"),
    }
}

fn create(title: String, day: String) -> Result<()> {
    let date_re = Regex::new(r"^\d{4}\d{2}\d{2}$").unwrap();

    if !date_re.is_match(&day) {
        panic!("the date {day} is not valid, correct format is YYYYmmdd");
    }

    let path = get_mtask_path(&day);

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("could not create a file for today");

    file.write_all(title.as_bytes()).unwrap();
    file.write_all("\n".as_bytes()).unwrap();
    println!("task added");

    Ok(())
}

fn get_mtask_dir() -> PathBuf {
    let mut path = get_home_dir();
    path.push(".mtasks");
    if !path.exists() {
        std::fs::create_dir(&path).expect("failed to create .mtask folder on $HOME dir");
    }
    path
}

fn get_mtask_path(day: &str) -> PathBuf {
    let mut path = get_mtask_dir();
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
