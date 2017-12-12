use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

fn read_line() -> String {
	print!("please input file name: ");
	let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).ok();
    return buf.split_whitespace().collect()
}

fn open_file() -> String {
	let path = Path::new("../Cargo.toml");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => (),
    }
    return s.to_string();
}


fn write_line(s: &str) {
	let path = Path::new("../Cargo.toml");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(s.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               Error::description(&why))
        },
        Ok(_) => (),
    }
}

fn create_file(s: &str) {
	let mut src = String::new();
	src = s.to_owned() + ".rs";

	let path = Path::new(src.as_str());
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all("".as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               Error::description(&why))
        },
        Ok(_) => (),
    }
}

fn main() {
	let mut string = String::new();
	let file_name = read_line();

	let name = "

[[bin]]
name = ";

	let path = "
path = ";

	string = open_file() + name + "\"" + file_name.as_str() + "\"" + path + "\"src/" + file_name.as_str() + ".rs\"";

	write_line(&string);
	create_file(&file_name);

   	print!("\n");
}
