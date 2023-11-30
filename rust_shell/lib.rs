use std::io;
use std::process::Command;

pub fn run_interface() {
    print!("> ");
    loop {
        let mut com = String::new();
        io::stdin()
            .read_line(&mut com)
            .expect("Failed to read input");
        let args: Vec<&str> = com.trim().split(' ').collect();
        if args[0] == "" {
            continue;
        }
        let mut cmd = String::from("/bin/");
        cmd.push_str(args[0]);
        let output = Command::new(cmd)
            .args(&args[1..])
            .output()
            .expect("failed to execute process");
        print!(
            "{}",
            String::from_utf8(output.stdout).expect("invalid bytes")
        );
        print!("> ");
    }
}
