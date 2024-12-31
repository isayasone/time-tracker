use assert_cmd::Command;
use testresult::TestResult;

#[test]
fn status_code_is_error_if_no_command_specified() -> TestResult {
    Command::cargo_bin("track")?.assert().failure();
    #[allow(unreachable_code)]
    Ok(())
}

#[test]

fn status_command_starts_tracking_time() -> TestResult {
    Command::cargo_bin("track")?.arg("start").assert().success();
    todo!("");
    #[allow(unreachable_code)]
    Ok(())
}

#[test]

fn stop_command_stops_tracking_time() -> TestResult {
    Command::cargo_bin("track")?.arg("start").assert().success();
    Command::cargo_bin("track")?.arg("stop").assert().success();
    todo!("");
    #[allow(unreachable_code)]
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
