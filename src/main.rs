mod client;
use std::env;

fn main() {
    let args: Vec<String> = env::args()
        .collect();
    
    if args.len() >= 3 {
        let passed_in: &Vec<String> = &args[1..args.len()].iter()
            .map(|arg| arg.clone())
            .collect();
        route(passed_in);
    }
}

fn route(args: &Vec<String>) {
    client::send(args[0].as_str(), args[1].as_str())
        .expect("Error: Could not send data!");
}