use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    // If user just types the name, show them how to use it
    if args.len() < 2 {
        println!("--- Vacation Planner CLI ---");
        println!("Usage: vacation-tool [item to pack or task to do]");
        return;
    }

    let task = &args[1..].join(" ");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("vacation_list.txt")
        .expect("Could not open your vacation list");

    if let Err(e) = writeln!(file, "[ ] {}", task) {
        eprintln!("Error saving task: {}", e);
    } else {
        println!("Added to vacation list: {}", task);
    }
}