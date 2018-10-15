use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //compiler tell me open return types let f: u32 = File::open("hello");

    let f = File::open("hello");

    /*
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
    */
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e)
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error)
        },
    };

    // let f = File::open("hello.txt").unwrap();
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let result = read_username_from_file();
    println!("{:?}", result);

    let result = read_username_from_file2();
    println!("{:?}", result);
    
    let result = read_username_from_file3();
    println!("{:?}", result);

    let result = read_username_from_file4();
    println!("{:?}", result);

    let f = File::open("hello.txt")?;

    println!("run here");
}

fn read_username_from_file() -> Result<String, io::Error> {

    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), 
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();

    // ? Operator
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

use std::fs;
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hellow.txt")
}
