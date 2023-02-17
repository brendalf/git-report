use std::process::Command;
use std::collections::HashMap;

#[macro_export]
macro_rules! run_git {
    ($prog:expr; $($x:tt),*)  =>  {{
        let mut cmd = Command::new($prog);
        $(cmd.arg($x);)*
        let cmd_output = cmd.output().expect("Invalid command");
        String::from_utf8(cmd_output.stdout).expect("Invalid utf8 output from command")
    }}
}


pub fn get_tracked_files() -> Vec<String> {
    let file_string: String = run_git!("git"; "ls-files");
    return file_string.lines().map(|x| x.to_string()).collect();
}

pub fn return_file_activity(files: &Vec<String>) -> HashMap<String, u32> {
    let mut output: HashMap<String,u32> = HashMap::new();
    for file in files {
        let commit_output: String = run_git!("git"; "rev-list", "--count", "HEAD", "--", file);
        output.insert(file.to_owned(), commit_output.trim().parse::<u32>().expect("Value wasn't a valid integer"));
    }
    return output;
}
