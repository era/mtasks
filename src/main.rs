use clap::Parser;
use clap::Subcommand;
use std::io::Write;
use std::path::PathBuf;

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
    Create { title: String },
    List {},
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Create { title } => create(title).unwrap(),
        Action::List {} => todo!(),
    }
}

fn create(title: String) -> Result<()> {
    let mut path = get_home_dir();
    path.push(".mtasks");
    if !path.exists() {
        std::fs::create_dir(&path).expect("failed to create .mtask folder on $HOME dir");
    }

    path.push(&get_curr_day());

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

fn get_curr_day() -> String {
    chrono::offset::Local::now()
        .format("task_%Y%m%d")
        .to_string()
}

fn get_home_dir() -> PathBuf {
    std::env::var_os("HOME")
        .expect("you need to have $HOME setup")
        .into()
}
