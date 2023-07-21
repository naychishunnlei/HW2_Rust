use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn area_no_args(){
    let mut cmd = Command::cargo_bin("circle").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
}

#[test]
fn area_args(){

    let r = "10";

    let mut cmd = Command::cargo_bin("circle").unwrap();
    cmd.arg(r).assert().success();
   
}

#[test]
fn area_args_with_stdout(){

    let r = "10";

    let r_cal = &r.to_string().parse().unwrap_or(0.0);
    let area =  3.1416 * r_cal * r_cal;
    let expected = format!("The area of the circle with radius = {r} is : {area}\n");

    let mut cmd = Command::cargo_bin("circle").unwrap();

    cmd.arg(r).assert().success().stdout(expected);
    
}