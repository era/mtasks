#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::path::Path;
    use tempdir::TempDir;
    #[test]
    fn test_help() {
        Command::cargo_bin("mtask")
            .unwrap()
            .arg("-h")
            .assert()
            .success()
            .stdout(predicate::str::contains("mtask"));
    }

    #[test]
    fn test_create_and_list_yesterday() {
        let day = chrono::offset::Local::now() - chrono::Duration::days(1);
        let day = day.format("%Y%m%d").to_string();
        let task = "such a cool day";
        let temp_dir = TempDir::new("mstak_fixture").expect("could not create tempdir");

        mtask(temp_dir.path())
            .arg("create")
            .arg("should not be there")
            .assert()
            .success();

        mtask(temp_dir.path())
            .arg("create")
            .arg(task)
            .arg("-d")
            .arg(day)
            .assert()
            .success();
        mtask(temp_dir.path())
            .arg("list")
            .arg("-l")
            .assert()
            .success()
            .stdout(predicate::str::contains(task));
    }

    #[test]
    fn test_create_and_list() {
        let day = chrono::offset::Local::now().format("%Y%m%d").to_string();
        let today_task = "such a cool day";
        let temp_dir = TempDir::new("mstak_fixture").expect("could not create tempdir");
        mtask(temp_dir.path())
            .arg("create")
            .arg(today_task)
            .assert()
            .success();
        mtask(temp_dir.path())
            .arg("list")
            .arg("-d")
            .arg(day)
            .assert()
            .success()
            .stdout(predicate::str::contains(today_task));
    }

    #[test]
    fn test_create_and_list_specific_date() {
        let day = "20221106";
        let task = "such a cool day";
        let temp_dir = TempDir::new("mstak_fixture").expect("could not create tempdir");
        mtask(temp_dir.path())
            .arg("create")
            .arg(task)
            .arg("-d")
            .arg(day)
            .assert()
            .success();
        mtask(temp_dir.path())
            .arg("list")
            .arg("-d")
            .arg(day)
            .assert()
            .success()
            .stdout(predicate::str::contains(task));
    }

    fn mtask(home: &Path) -> Command {
        let mut cmd = Command::cargo_bin("mtask").unwrap();
        cmd.env("HOME", home);
        cmd
    }
}
