use figlet_rs::FIGfont;
use std::io::{self, Write};

fn main() {
  loop {
    let mut input = String::new();
    print!("Text here > ");

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input) //mutable soalnya
        .expect("Cannot read line");

    if input.trim().eq("/exit") {
      break;
    } else {
      cetak_banner(input);
    }
  }
}

fn cetak_banner(input: String) {
  let standard_font = FIGfont::standand().unwrap();
  let figure = standard_font.convert(input.trim());
  assert!(figure.is_some()); 
  println!("{}", figure.unwrap());
}
