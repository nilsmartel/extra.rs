use std::io::{self, Read};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("two arguments required");
        process::exit(0);
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer);

        print_extract(buffer.as_str(), &args[1], &args[2]);
    }
}

fn print_extract(txt: &str, a: &String, b: &String) {
    if txt.contains(a) {
        let i = txt.find(a).unwrap()+a.len();
        let txt2 = &txt[i..];
        if txt2.contains(b) {
            let j = txt2.find(b).unwrap();
            println!("{}", &txt2[..j] );
            print_extract(&txt2[j+b.len()..], &a, &b);
        }
    }
}
