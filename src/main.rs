use std::process::Command;

fn main() {
    for i in (1..=25) {
        let day = format!("day{:0>2}", i);
        let cmd = Command::new("cargo")
            .args(&["run", "--bin", &day])
            .output()
            .unwrap();
        let output = String::from_utf8(cmd.stdout).unwrap();
        println!("Day {}:\n{}", day, output);
    }
    println!("Hello, world!");
}
