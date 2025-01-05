use std::path::PathBuf;

use assert_cmd::Command;

use assert_fs::TempDir;
use testresult::TestResult;

#[test]
fn status_code_is_error_if_no_command_specified() -> TestResult {
    Command::cargo_bin("track")?.assert().failure();
    #[allow(unreachable_code)]
    Ok(())
}

fn tracking_paths() -> (TempDir, PathBuf, PathBuf) {
    let temp = TempDir::new().unwrap();
    let lockfile = temp.path().join("lockfile");
    let db = temp.path().join("db.json");
    (temp, lockfile, db)
}

#[test]
fn start_command_tracking_time() -> TestResult {
    let (tempdir, lockfile, db) = tracking_paths();

    assert!(!lockfile.exists(), "Lockfile should not exist yet.");
    assert!(!db.exists(), "Database file should not exist yet.");

    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg("db.json")
        .arg("--lockfile")
        .arg(&lockfile)
        .arg("start")
        .assert()
        .success();

    assert!(lockfile.exists());
    // assert!(db.exists());
    tempdir.close()?;
    Ok(())
}

#[test]

fn stop_command_stops_tracking_time() -> TestResult {
    let (tempdir, lockfile, db) = tracking_paths();

    assert!(!lockfile.exists(), "Lockfile should not exist yet.");
    assert!(!db.exists(), "Database file should not exist yet.");

    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg("db.json")
        .arg("--lockfile")
        .arg(&lockfile)
        .arg("start")
        .assert()
        .success();

    Command::cargo_bin("track")?
        .arg("--db-dir")
        .arg("db.json")
        .arg("--lockfile")
        .arg(&lockfile)
        .arg("stop")
        .assert()
        .success();

    assert!(!lockfile.exists());
    // assert!(db.exists());
    tempdir.close()?;
    Ok(())
}

#[test]

fn report_command_generates_report() -> TestResult {
    Command::cargo_bin("track")?.arg("start").assert().success();
    Command::cargo_bin("track")?.arg("stop").assert().success();
    Command::cargo_bin("track")?
        .arg("report")
        .assert()
        .stdout("00:00:00/n")
        .success();

    todo!("");
    #[allow(unreachable_code)]
    Ok(())
}
