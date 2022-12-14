use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

fn main() {

    // Run the executable with the specified parameters
    let mut command = Command::new("C:\\Tools\\SteamCMD\\steamcmd.exe")
        .arg("+login")
        .arg("anonymous")
        .arg("+workshop_download_item")
        .arg("294100")
        .arg("2897974516")
        .arg("+quit")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let reader = BufReader::new(command.stdout.take().unwrap());

    // Wait for success message in the output
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        if line.contains("Success") {
            println!("Found success!");
        }
    }

    println!("Made it out");
}

