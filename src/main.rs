use std::process::Command;

fn main() {
  let output = Command::new("git")
    .arg("branch")
    .output()
    .expect("something broke");

  let lines = match String::from_utf8(output.stdout) {
    Ok(val) => val,
    _ => panic!("Something went wrong"),
  };

  if lines.is_empty() {
    println!("There are no branches in this directory.");
  } else {
    println!("{}", lines);
  }

}
