use std::io;
use std::process::Command;

pub fn run_interface() {
    loop {
        print!("> ");
        let mut com = String::new();
        io::stdin()
            .read_line(&mut com)
            .expect("Failed to read input");
        let args: Vec<&str> = com.trim().split(' ').filter(|s| !s.is_empty()).collect();
        if args.len() == 0 {
            continue;
        }
        let mut process = Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .expect("failed to execute process");
        let ecode = process.wait().expect("failed to wait on child");
        print!("{}", ecode);
    }
}
