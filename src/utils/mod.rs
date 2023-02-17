use std::process::Command;

#[macro_export]
macro_rules! run_cmd {
    ($prog:expr, $($x:tt)*) =>  {{
        let mut cmd = Command::new($prog);
        $(cmd.arg($x);)*
        let cmd_output = cmd.output().expect("Invalid command");
        String::from_utf8(cmd_output.stdout).expect("Invalid utf8 output from command")
    }}
}


pub fn get_tracked_files() -> Vec<String> {
    let file_string: String = run_cmd!("git", "ls-files");
    return file_string.lines().map(|x| x.to_string()).collect();
}

