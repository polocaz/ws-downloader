use std::env;
use std::fs::{self, File, OpenOptions};

use std::io::{self, BufRead, BufReader, Read, Write, Error};
mod htmlutils;
mod downloader;

use std::path::{Path, PathBuf};

use crate::downloader::process_download;

// Defaults cmdpath and content path
pub struct ConfigStruct {
    cmdpath: String,
    contentpath: String,
    appid: String,
}

struct ModInfo {
    path: String,
    file_name: String,
    start_tag: String,
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
    println!("App id: {}", appid == "294100");
    
    if appid == "294100" {
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
        None => {
            println!("Got nothing");
            return},
    };
    let mut data: String;
    // Grab mod name
    if appid != "233860" {
        data = match get_mod_name(&info.path, &info.file_name, &info.start_tag) {
            Some(s) => {
                println!("{}", s);
                s
            },
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

fn read_config_file(filename: &str) -> Result<(String, String), Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut pathcmd = String::new();
    let mut pathcontent = String::new();

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("PATHCMD=") {
            pathcmd = line["PATHCMD=".len()..].to_owned();
        } else if line.starts_with("PATHCONTENT=") {
            pathcontent = line["PATHCONTENT=".len()..].to_owned();
        }
    }

    Ok((pathcmd, pathcontent))
}

fn start_rename(directory: &str, appid: &String) {
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // rename the directory
            println!("{}", path.to_str().unwrap());
            rename_folder(&path.to_str().unwrap().to_owned(), &appid)
        }
    }
}

fn main() {
    // Get cmdline args mainpath, contentpath, game app id

    let args: Vec<String> = env::args().collect();

    // 233860
    let mut appidarg = "294100".to_owned();

    if args.len() < 2 {
        println!(
            "Usage: scmd-downloader.exe <arg> <appid> or scmd-downloader.exe <arg> for rimworld"
        );
        return;
    } else if args.len() == 3 {
        appidarg = args[2].to_owned();
    }

    let arg = &args[1];

    // Read Config
    let mut config = ConfigStruct {
        cmdpath: String::from("C:\\Tools\\SteamCMD\\"),
        contentpath: String::from("steamapps\\workshop\\content\\"),
        appid: appidarg
    };

    let cfgfile = read_config_file("data\\config.txt");
        

    if let Ok(content) = cfgfile {
        if content.0 != config.cmdpath  {
            config.cmdpath = content.0;
        } 
        else if content.1 != config.contentpath {
            config.contentpath = content.1;
        }
    }

    if arg == "rename" {
        println!("Starting renaming");
        start_rename(".\\data\\mods", &config.appid);
    } else if arg == "download" {
        println!("Starting downloads");
        process_download("data\\urls.txt", config);
    } else if arg == "collection" {
        println!("Building url list");
        htmlutils::build_url_list("data\\collection.html".to_owned());
    }
}
