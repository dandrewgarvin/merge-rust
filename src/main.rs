use std::env;
use std::process::Command;

fn main() {
  let args: Vec<String> = env::args().collect();
  let source_branch = &args[1];

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
    let mut current_branch = String::new();

    for line in lines.lines() {
      if line.contains("* ") {
        current_branch = line.to_string();
      }
    }

    current_branch = current_branch[2..].to_string();

    if &current_branch != source_branch {
      println!("Checking out to {}", source_branch);

      Command::new("git")
        .arg("checkout")
        .arg(source_branch)
        .output()
        .expect("Unable to checkout");

      println!("Pulling code...");
      let pull = Command::new("git")
        .arg("pull")
        .output()
        .expect("Unable to pull new code");

      let pull = match String::from_utf8(pull.stdout) {
        Ok(val) => val,
        _ => panic!("Something went wrong"),
      };

      for line in pull.lines() {
        println!("{}", line);
      }

      println!("Checking back into {}", current_branch);
      Command::new("git")
        .arg("checkout")
        .arg(current_branch)
        .output()
        .expect("Unable to checkout");

      println!("Merging code...");
      let merge = Command::new("git")
        .arg("merge")
        .arg(source_branch)
        .output()
        .expect("Unable to merge code");

      let merge = match String::from_utf8(merge.stdout) {
        Ok(val) => val,
        _ => panic!("Something went wrong"),
      };

      for line in merge.lines() {
        println!("{}", line);
      }
    }
  }
}
