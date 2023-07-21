use assert_cmd::Command;

#[test]
fn players_no_args() {

    let mut cmd = Command::cargo_bin("list_players").unwrap();

    let expected = format!("Player 1: N/A\nPlayer 2: N/A\n");
    cmd.assert().success().stdout(expected);
}

#[test]
fn players_with_one_arg() {

    let arg1= "Mike";
    let expected = format!("Player 1: {arg1}\nPlayer 2: N/A\n");

    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.arg(arg1);
    cmd.assert().success().stdout(expected);
}

#[test]
fn players_with_two_args() {

    let arg1= "Mike";
    let arg2 = "John";
    let expected = format!("Player 1: {arg1}\nPlayer 2: {arg2}\n");

    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.args(vec![arg1, arg2]);
    cmd.assert().success().stdout(expected);
}

#[test]
fn list_players_with_extra_args() {

    let arg1= "Mike";
    let arg2 = "John";
    let arg3 = "Jane";
    let expected = format!("Player 1: {arg1}\nPlayer 2: {arg2}\n");

    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.args(vec![arg1, arg2, arg3]);
    cmd.assert().success().stdout(expected);
}