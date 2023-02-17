use regex::Regex;
use std::{process::Command, collections::HashMap};
mod utils;
use utils::{return_file_activity, get_tracked_files};

fn summarize() {
    let ls_output = Command::new("git").args(["ls-files"]).output().expect("Not a git repository");
    let ls_output_stdout = String::from_utf8(ls_output.stdout).unwrap();

    let email_re = Regex::new(r"<[\w@._-]+>").unwrap();

    let mut commits_by_email: HashMap<String, u16> = HashMap::new();

    let files = ls_output_stdout.lines();

    for file in files {
        // Ignore some files?
        if file == "LICENSE" {
            continue;
        }

        let blame_output = Command::new("git").args(["blame", file, "-e"]).output().expect("Error reading the file");
        let blame_output_stdout = String::from_utf8(blame_output.stdout).unwrap();

        for line in blame_output_stdout.lines() {
            let email = &email_re.captures_iter(&line).next().unwrap()[0];
            
            if email == "<not.committed.yet>" {
                continue;
            }

            commits_by_email.entry(email.to_string()).and_modify(|v| {*v += 1}).or_insert(1);
        }
    }

    println!("{commits_by_email:?}");
}

fn main() {
    let files: Vec<String> = get_tracked_files();
    let activity: HashMap<String, u32> = return_file_activity(&files);
    println!("File activity: {activity:?}");
    summarize();
}
