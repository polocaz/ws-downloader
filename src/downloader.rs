use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};

fn download_item(modid: &str, cmdpath: &str, appid: &str) -> bool {
    // Run the executable with the specified parameters
    let mut command = Command::new(cmdpath.to_owned() + "steamcmd.exe")
        .arg("+login")
        .arg("anonymous")
        .arg("+workshop_download_item")
        .arg(appid)
        .arg(modid)
        .arg("+quit")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let reader = BufReader::new(command.stdout.take().unwrap());

    // We want to return the line of the failed urls

    // Wait for success message in the output
    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains("Success") {
            println!("Found success!");
            return true;
        }
    }

    false
}

fn extract_id(url: &str) -> String {
    // Split the URL into multiple substrings using the "&searchtext=" separator
    let parts: Vec<&str> = url.split("?id=").collect();

    // Return the second substring, which is the part of the URL that we want to extract
    let id: &str = parts.get(1).map(|x| *x).unwrap_or("");

    if id.contains('&') {
        let parts2: Vec<&str> = id.split("&").collect();

        parts2.get(0).map(|x| *x).unwrap_or("").to_owned()
    } else {
        id.to_owned()
    }
}

fn get_urls(filename: &str) -> Vec<String> {
    // Construct the path to the file by joining the filename with the current working directory
    // let path = std::env::current_dir().unwrap().join(filename);

    // Open the file in read-only mode
    let file = fs::File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Read each line from the file and store it in the `urls` vector
    let mut urls = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line from file");
        urls.push(line);
    }

    urls
}

fn remove_done(filename: &str, lines_to_remove: Vec<usize>) -> io::Result<()> {
    // Open the file in read-write mode
    let mut file = OpenOptions::new().read(true).write(true).open(filename)?;

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Remove the lines that we want to delete
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut modified_contents = String::new();
    for (i, line) in lines.iter().enumerate() {
        if !lines_to_remove.contains(&i) {
            modified_contents.push_str(line);
            modified_contents.push('\n');
        }
    }

    // Open the file in write mode
    let mut file = File::create(filename)?;

    // Write the modified contents back to the file
    file.write_all(modified_contents.as_bytes())?;

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////
/// 

pub fn process_download(urlslocation: &str ) {
    // Grab urls from data file
    let urls: Vec<String> = get_urls(urlslocation);
    let mut succeeded: Vec<usize> = Vec::new();
    let mut linecount = 0;
    //Get id for each url
    for url in urls.iter() {
        let id = extract_id(url);
        if id.is_empty() {
            println!(
                "Tried to read a url and got an empty line{}: {}",
                linecount, url
            );
            continue;
        }
        println!("ID: {}", id);

        // Download each id
        let res = download_item(&id, &PATHCMD, &appid);
        if res == false {
            println!("Failed to download id: {}", id);
        } else {
            // Save for later so we can remove from the urls file
            succeeded.push(linecount);
        }
        linecount += 1;
    }

    if succeeded.is_empty() {
        println!("Failed to download any mods, check your urls.txt file and try again");
        return;
    }
    // Clean up finished urls

    match remove_done(".\\data\\urls.txt", succeeded) {
        Ok(_) => {
            println!("Cleaned up urls file successfully");
            println!(
                "Go here: {} to get your mods",
                PATHCMD.to_owned() + _PATHCONTENT + &appid
            );
        }
        Err(_) => println!("Failed to clean up urls file"),
    };
}