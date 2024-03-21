mod tasks;
mod utils;
use std::env;
use utils::input;

fn longest(a1: &str, a2: &str) -> String {
    let bh = a1
        .chars()
        .chain(a2.chars())
        .collect::<std::collections::BTreeSet<_>>();
    bh.iter().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).unwrap().as_str() {
        "2" => {
            let lines = input::read_file_lines("../inputs/z2.txt");
            tasks::z2::solve(lines);
        }
        "4" => {
            let lines = input::read_file_lines("../inputs/z4.txt");
            tasks::z4::solve(lines);
        }
        "8" => {
            // let lines = input::read_file_lines("../inputs/z8-debug.txt");
            // let lines = input::read_file_lines("../inputs/z8-debug2.txt");
            let lines = input::read_file_lines("../inputs/z8.txt");
            tasks::z8::solve(lines);
        }
        "9" => {
            let lines = input::read_file_lines("../inputs/z9.txt");
            tasks::z9::solve(lines);
        }
        "10" => {
            let lines = input::read_file_lines("../inputs/z10-debug.txt");
            //let lines = input::read_file_lines("../inputs/z10.txt");
            tasks::z10::solve(&lines);
        }
        "11" => {
            // let lines = input::read_file_lines("../inputs/z11-debug.txt");
            let lines = input::read_file_lines("../inputs/z11.txt");
            tasks::z11::solve(&lines);
        }
        _ => println!("No matching args"),
    }
}
