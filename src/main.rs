use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

static PATHCMD: &str = "C:\\Tools\\SteamCMD\\";

static _PATHCONTENT: &str = "steamapps\\workshop\\content\\";

struct ModInfo {
    path: String,
    file_name: String,
    start_tag: String,
}

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

// Get rid of the characters that are not allowed in the folder name
fn clean_folder_name(s: &str) -> String {
    let stripped: String = s
        .chars()
        .filter(|c| !c.is_ascii_control())
        .filter(|c| *c != '\\')
        .filter(|c| *c != '/')
        .filter(|c| *c != ':')
        .filter(|c| *c != '*')
        .filter(|c| *c != '?')
        .filter(|c| *c != '"')
        .filter(|c| *c != '<')
        .filter(|c| *c != '>')
        .filter(|c| *c != '|')
        .filter(|c| *c != ' ')
        .collect();

    stripped
}

// Get mod name for a rimworld mod
fn get_mod_name(path: &str, file_name: &str, start_tag: &str) -> Option<String> {
    // Try to open the file
    let full_path: String = format!("{}\\{}", path, file_name);
    let file: File = match File::open(&full_path) {
        Ok(f) => f,
        Err(_) => {
            println!("Could not open File ERROR:");
            return None;
        }
    };

    let reader: BufReader<File> = BufReader::new(file);

    // Go through each line and find the name tag for the mod
    for line in reader.lines() {
        // Check if we read the line correctly
        let line: String = match line {
            Ok(s) => s,
            Err(_) => {
                println!("Failed to read line");
                return None;
            }
        };

        if !line.contains(start_tag) {
            continue;
        }

        // Get index for the end of the first tag
        let first: usize = match line.find('>') {
            Some(n) => n,
            None => {
                println!("Failed to find the start tag in file: {}", &full_path);
                return None;
            }
        };

        // Get index for start of end tag
        let second: usize = match line[&first + 1..].find('<') {
            Some(n) => n,
            None => {
                println!("Failed to find the end tag in file: {}", &full_path);
                return None;
            }
        };

        // Use both to get the name of the mod
        match line.get(first + 1..first + second + 1) {
            Some(s) => return Some(s.to_string()),
            None => {
                println!("Failed to get name from file: {}", &full_path);
                return None;
            }
        };
    }

    println!("No name found in file: {}", full_path);
    None
}

fn find_file_from_ext(dir: &String, ext: &String) -> String {
    for entry in Path::new(dir)
        .read_dir()
        .unwrap()
        .filter_map(|entry| entry.ok())
    {
        if entry.path().extension() == Some(ext.as_ref()) {
            println!("Found file: {}", entry.path().display());
            let whole_name = entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();

            let trimmed_name = match whole_name.rfind('.') {
                Some(i) => &whole_name[..i],
                None => &whole_name,
            };

            if trimmed_name.find('.').is_none() {
                return trimmed_name.to_owned();
            }
        }
    }

    "".to_owned()
    // Path::new(dir)
    //     .read_dir()
    //     .unwrap()
    //     .filter_map(|entry| entry.ok())
    //     .find(|entry| entry.path().extension() == Some(ext.as_ref()))
    //     .map(|entry| entry.path())
    //     .unwrap_or_else(|| {
    //         println!("Error: Could not find .mod file in {}", dir);
    //         PathBuf::new()
    //     })
}

fn find_kenshimod_file(dir: &String) -> Option<String> {
    // Check if the directory exists
    if !Path::new(dir).exists() || !Path::new(dir).is_dir() {
        println!("Error: {} is not a directory", dir);
        return None;
    }

    // Search the directory for a .mod file
    let mod_file = find_file_from_ext(dir, &"mod".to_owned());

    if !mod_file.is_empty() {
        return Some(mod_file);
    }

    None
}

fn get_modinfo(oldpath: &String, appid: &String) -> Option<ModInfo> {
    if appid == "291400" {
        Some(ModInfo {
            path: oldpath.to_owned() + "\\About",
            file_name: "About.xml".to_owned(),
            start_tag: "<name>".to_owned(),
        })
    } else if appid == "233860" {
        // Get the name of the kenshi mod .mod file so we can rename the folder
        let kmod_name = match find_kenshimod_file(&oldpath.to_owned()) {
            Some(filename) => filename,
            None => return None,
        };
        Some(ModInfo {
            path: oldpath.to_owned(),
            file_name: kmod_name,
            start_tag: "<name>".to_owned(),
        })
    } else {
        None
    }
}

fn rename_folder(old: &String, appid: &String) {
    // Defaulted for rimworld

    let info = match get_modinfo(&old.to_owned(), appid) {
        Some(info) => info,
        None => return,
    };
    let mut data: String;
    // Grab mod name
    if appid != "233860" {
        data = match get_mod_name(&info.path, &info.file_name, &info.start_tag) {
            Some(s) => s,
            None => {
                println!(
                    "Failed to get the mod name, rename unsuccessful {}",
                    &info.path
                );
                return;
            }
        };

        // Strip bad characters
        data = clean_folder_name(&data);
    } else {
        data = info.file_name;
    }

    // Rename the folder
    let modpath = Path::new(&old);
    let new_path = modpath.with_file_name(&data);

    match fs::rename(&modpath, &new_path) {
        Ok(_) => return,
        Err(e) => {
            println!(
                "Failed to rename {} to {} error3: {}",
                &modpath.to_str().unwrap(),
                &new_path.to_str().unwrap(),
                e.to_string()
            );
        }
    }
}

fn start_rename(directory: &str, appid: &String) {
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // rename the directory
            rename_folder(&path.to_str().unwrap().to_owned(), &appid)
        }
    }
}

fn main() {
    // Get cmdline args mainpath, contentpath, game app id

    let args: Vec<String> = env::args().collect();

    // 233860
    let mut appid = "294100".to_owned();

    if args.len() < 2 {
        println!(
            "Usage: scmd-downloader.exe <arg> <appid> or scmd-downloader.exe <arg> for rimworld"
        );
        return;
    } else if args.len() == 3 {
        appid = args[2].to_owned();
    }

    let arg = &args[1];

    if arg == "rename" {
        println!("Starting renaming");
        start_rename(".\\data\\mods", &appid);
    } else if arg == "download" {
        println!("Starting downloads");
        // Grab urls from data file
        let urls: Vec<String> = get_urls(".\\data\\urls.txt");
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
}
