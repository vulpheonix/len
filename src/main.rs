//! Contains the entire source code of len
//!
//! Copyright (C) 2022 Vulpheonix
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crossterm::style::Stylize;
use crossterm::{cursor, ExecutableCommand, QueueableCommand};
use std::cmp::Ordering;
use std::env::args;
use std::fs;
use std::io::{stdout, Write};
use std::path::Path;
use std::process::exit;

/// A constant value required to compute byte size
const THOUSAND: f64 = 1000.0;

/// Stores the total number of files in a directory (recursively)
static mut FILE_COUNT: i32 = 0;
/// Stores the total number of sub-directories in a directory (recursively)
static mut DIR_COUNT: i32 = 0;

/// Displays directory size, total number of files and sub-directories it contains and the Size of each of its items
unsafe fn show_dir_info(dir: &Path) {
    let mut stdout = stdout();

    stdout.execute(cursor::Hide).unwrap();
    stdout.queue(cursor::SavePosition).unwrap();
    stdout
        .write_all(
            format!(
                "{}",
                "computing (this is not a deadlock) ..."
                    .bold()
                    .white()
                    .italic()
            )
            .as_bytes(),
        )
        .unwrap();
    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.flush().unwrap();

    FILE_COUNT = 0;
    DIR_COUNT = 0;
    let dir_info = get_dir_info(&dir, 0);
    let dir_name = match dir.file_name() {
        Some(_name) => match _name.to_str() {
            Some(_name_str) => _name_str,
            None => ".",
        },
        None => ".",
    };

    stdout
        .write_all("                                      ".as_bytes())
        .unwrap();
    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.execute(cursor::Show).unwrap();

    if !dir_name.eq(".") {
        print!("{} ", dir_name.blue().bold());
    }
    println!("{}", format_bytes(dir_info.0 as f64).underlined());
    println!(
        "{} files(rec), {} dirs(rec)",
        dir_info.1.to_string().bold().white(),
        dir_info.2.to_string().bold().white()
    );
    println!(
        "Total {} Items",
        (dir_info.1 + dir_info.2).to_string().bold().white()
    );
    show_file_list(dir);
}

/// Accordingly calls the respective function to display file weight depending upon path's type
unsafe fn analyse_file(file: &Path) {
    if file.is_dir() {
        show_dir_info(file);
    } else {
        println!("{}", get_file_info(file));
    }
}

/// Returns a formatted file size text
fn get_file_info(file: &Path) -> String {
    let size = file.metadata().unwrap().len();
    format_bytes(size as f64)
}

/// Displays the items in a directory along with their weights
unsafe fn show_file_list(dir: &Path) {
    let mut file_list = vec![];
    let mut dir_list = vec![];
    for path in fs::read_dir(dir).unwrap() {
        let file = path.unwrap().path();
        if file.is_file() {
            file_list.push(file);
        } else {
            dir_list.push(file);
        }
    }
    dir_list.sort_by(|path1, path2| {
        let name1 = path1.file_name().unwrap().to_str().unwrap();
        let name2 = path2.file_name().unwrap().to_str().unwrap();
        return match name1.cmp(name2) {
            Ordering::Greater => Ordering::Equal,
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => Ordering::Less,
        };
    });
    file_list.sort_by(|path1, path2| {
        let name1 = path1.file_name().unwrap().to_str().unwrap();
        let name2 = path2.file_name().unwrap().to_str().unwrap();
        return match name1.cmp(name2) {
            Ordering::Greater => Ordering::Equal,
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => Ordering::Less,
        };
    });
    println!("{}", "----------Contents----------".italic());
    for path in dir_list {
        println!(
            "|- {: <10} | {: <10}",
            path.file_name().unwrap().to_str().unwrap().bold().grey(),
            format_bytes(get_dir_info(&path, 0).0 as f64)
        );
    }
    for path in file_list {
        println!(
            "|- {: <10} | {: <10}",
            path.file_name().unwrap().to_str().unwrap().bold().cyan(),
            get_file_info(&path)
        );
    }
}

/// A recursive function to get total weight of the directory in bytes and to update the total items it contains
unsafe fn get_dir_info(dir: &Path, mut bytes: u64) -> (u64, u32, u32) {
    if !dir.exists() {
        return (0, 0, 0);
    }
    let contents = fs::read_dir(dir);
    for path in contents.unwrap() {
        let file = path.unwrap().path();
        if file.is_file() {
            bytes += file.metadata().unwrap().len();
            FILE_COUNT += 1;
        } else {
            bytes += get_dir_info(&file, 0).0;
            DIR_COUNT += 1;
        }
    }
    (bytes, FILE_COUNT as u32, DIR_COUNT as u32)
}

/// Returns the formatted bytes info
fn format_bytes(bytes: f64) -> String {
    let suffix = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let power = (bytes.log10() / THOUSAND.log10()).floor() as usize;
    if power > 2 {
        return format!(
            "{} {}",
            format!("{:.1}", (bytes / THOUSAND.powf(power as f64)))
                .to_string()
                .white()
                .bold(),
            suffix[power].to_string().dark_red().italic().bold()
        );
    }
    return format!(
        "{} {}",
        format!("{:.1}", (bytes / THOUSAND.powf(power as f64)))
            .to_string()
            .white()
            .bold(),
        suffix[power].to_string().blue().italic().bold()
    );
}

/// Searches for file names in the dir path provided ending with the ext
fn analyse_wild_card(dir: &Path, ext: &str) {
    let mut file_count: u32 = 0;
    let mut file_list = vec![];
    for path in fs::read_dir(&dir).unwrap() {
        let file = path.unwrap().path();
        if file.is_file() {
            let name = file.file_name().unwrap().to_str().unwrap();
            if name.ends_with(&ext) {
                file_list.push(file);
                file_count += 1;
            }
        }
    }
    file_list.sort_by(|path1, path2| {
        let name1 = path1.file_name().unwrap().to_str().unwrap();
        let name2 = path2.file_name().unwrap().to_str().unwrap();
        return match name1.cmp(name2) {
            Ordering::Greater => Ordering::Equal,
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => Ordering::Less,
        };
    });
    let mut total_bytes: u64 = 0;
    for path in file_list {
        total_bytes += path.metadata().unwrap().len();
        println!(
            "|- {: <10} | {: <10}",
            path.file_name().unwrap().to_str().unwrap().bold().cyan(),
            get_file_info(&path)
        );
    }
    println!(
        "{} files found, weight {}",
        file_count.to_string().as_str().bold().white(),
        format_bytes(total_bytes as f64)
    );
}

/// Searches for matching file names in the dir path provided containing text
unsafe fn analyse_text(dir: &Path, text: &str) {
    let mut file_count: u32 = 0;
    let mut dir_count: u32 = 0;
    let mut dir_list = vec![];
    let mut file_list = vec![];
    for path in fs::read_dir(&dir).unwrap() {
        let file = path.unwrap().path();
        let name = file.file_name().unwrap().to_str().unwrap();
        if name.contains(text) {
            if file.is_file() {
                file_list.push(file);
                file_count += 1;
            } else {
                dir_list.push(file);
                dir_count += 1;
            }
        }
    }
    dir_list.sort_by(|path1, path2| {
        let name1 = path1.file_name().unwrap().to_str().unwrap();
        let name2 = path2.file_name().unwrap().to_str().unwrap();
        return match name1.cmp(name2) {
            Ordering::Greater => Ordering::Equal,
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => Ordering::Less,
        };
    });
    file_list.sort_by(|path1, path2| {
        let name1 = path1.file_name().unwrap().to_str().unwrap();
        let name2 = path2.file_name().unwrap().to_str().unwrap();
        return match name1.cmp(name2) {
            Ordering::Greater => Ordering::Equal,
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => Ordering::Less,
        };
    });
    let mut total_bytes: u64 = 0;
    for path in dir_list {
        let dir_info = get_dir_info(&path, 0);
        total_bytes += dir_info.0;
        println!(
            "|- {: <10} | {: <10}",
            path.file_name().unwrap().to_str().unwrap().bold().grey(),
            format_bytes(dir_info.0 as f64)
        );
    }
    for path in file_list {
        total_bytes += path.metadata().unwrap().len();
        println!(
            "|- {: <10} | {: <10}",
            path.file_name().unwrap().to_str().unwrap().bold().cyan(),
            get_file_info(&path)
        );
    }
    println!(
        "{} items found, weight {}",
        (file_count + dir_count).to_string().as_str().bold().white(),
        format_bytes(total_bytes as f64)
    );
}

/// Accordingly calls the respective part of the program by analysing the argument
unsafe fn analyse_arg(arg: &str) {
    let temp_file = Path::new(&arg);
    if is_wild_card(&arg) {
        analyse_wild_card(Path::new("."), &arg[1..]);
    } else if temp_file.exists() {
        analyse_file(temp_file);
    } else if arg.eq("--version") {
        println!("{} {}", "len".dark_cyan().bold(), "v0.1.0".bold().white());
    } else if arg.eq("--help") || arg.eq("-?") {
        println!(
            "{} {}",
            "Get file weights right into your Terminal with".bold(),
            "len".blue().bold()
        );
        println!("Usage:\t{}", "len (same as len .)".bold());
        println!("      \t{}", "len path-to-dir".bold());
        println!("      \t{}", "len path-to-file".bold());
        println!("      \t{}", "len wild-card".bold());
        println!("      \t{}", "len search-text".bold());
        println!("      \t{}", "len path-to-dir wild-card".bold());
        println!("      \t{}", "len path-to-dir search-text".bold());
        println!("{}", "brief description".italic());
        println!("{} {}", "#".bold(), "Using wild-card *".bold().dark_cyan());
        println!("{} //{}", "len \"*.mp3\"".bold(), "Remember, to wrap your query in double quotes while working with wild card option. Only asterisk(*) is supported at the moment.".italic());
        println!("{} {}", "#".bold(), "Using search-text".bold().dark_cyan());
        println!(
            "{} //{}",
            "len the".bold(),
            "Lists All items in the current directory which have \"the\" in their names.".italic()
        );
        println!(
            "{} {}",
            "For updates, keep a track on".bold(),
            "https://github.com/vulpheonix/len".red().bold()
        );
    } else {
        analyse_text(Path::new("."), &arg);
    }
}

/// Returns if the argument is actually a wild card
fn is_wild_card(arg: &str) -> bool {
    arg.starts_with("*")
}

/// The entry point of the program
fn main() {
    unsafe {
        match args().nth(1) {
            Some(_arg) => match args().nth(2) {
                Some(_arg2) => {
                    let temp_file = Path::new(&_arg);
                    if temp_file.exists() && temp_file.is_dir() {
                        if is_wild_card(_arg2.as_str()) {
                            analyse_wild_card(temp_file, &_arg2[1..]);
                        } else {
                            analyse_text(temp_file, &_arg2);
                        }
                    } else {
                        println!("{} does not exists!", _arg.bold());
                        exit(1);
                    }
                }
                None => analyse_arg(&_arg),
            },
            None => analyse_arg("."),
        }
    }
}
