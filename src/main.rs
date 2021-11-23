extern crate rasciigraph;
use rasciigraph::{plot, Config};

use std::io;

fn main() -> io::Result<()> {
    // init variables
    let mut data = vec![0.0; 0];
    let mut buf = String::new();
    let mut size_read: usize = 1;
    // set up + config
    let stdin = io::stdin();
    stdin.lock();
    let mut config = Config::default().with_height(15).with_offset(4);

    while size_read > 0 {
        size_read = stdin.read_line(&mut buf)?;
        if size_read == 0 {
            break;
        }
        // parse out f64 and append it to data
        let value: f64 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to parse number from input {}", buf),
                ))
            }
        };
        buf.clear();
        // append to data, print graph
        data.push(value);
        println!(
            "{}",
            plot(
                data.to_vec(),
                Config::default().with_height(15).with_offset(6)
            )
        );
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    Ok(())
}
