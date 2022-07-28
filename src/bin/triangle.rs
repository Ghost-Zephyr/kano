
use kano::Kano;

fn main() {
    let mut kano = match Kano::new() {
        Err(err) => {eprintln!("{}", err);return},
        Ok(k) => k,
    };
    println!("Hello, world!");
}
