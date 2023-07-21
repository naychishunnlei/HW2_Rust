use assert_cmd::Command;

#[test]
fn fah_to_cel(){

    let fah = ["-f", "50"];
    let mut cmd = Command::cargo_bin("temperature").unwrap();
    cmd.args(fah).assert().success();
}

#[test]
fn cel_to_fah(){

    let cel = ["-c", "100"];
    let mut cmd = Command::cargo_bin("temperature").unwrap();
    cmd.args(cel).assert().success();
}