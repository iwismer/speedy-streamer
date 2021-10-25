use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    // for argument in env::args() {
    //     println!("{}", argument);
    // }
    if env::args().len() != 4 {
        panic!(
            "Invalid number of arguments. Expected 3, found {}",
            env::args().len() - 1
        );
    }
    let stream = TcpStream::connect(format!(
        "{}:{}",
        env::args().nth(1).unwrap(),
        env::args().nth(2).unwrap()
    ))
    .expect("Unable to connect to the reader.");
    let reader = BufReader::new(stream);
    let mut output = OpenOptions::new()
        .create(true)
        .append(true)
        .open(env::args().nth(3).unwrap())
        .expect("Unable to write to file");
    // let mut output = File::create(env::args().nth(3).unwrap()).expect("Unable to write to file");
    for line in reader.lines() {
        let safe_line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        println!("{}", &safe_line);
        writeln!(output, "{}", &safe_line).expect("Unable to write to file.");
    }
}
