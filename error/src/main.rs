// use core::panic;


// fn main(){
//     // panic!("hell answers to mee")
// let v = vec![1, 2, 3];

//     v[99];
// }
// to find the backtrace the ideal way is to run the program with env variable as RUST_BACKTRACE=1   cargo run


// use std::{fs::File, println};

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let file= match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("Problem opening the file: {:?}", error);
//         }

//     };
//     println!("file opened successfully: {:?}", file);
// }
// use std::fs::File;
// use std::io::ErrorKind;
// use std::println;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             _ => {
//                 panic!("Problem opening the file: {error:?}");
//             }
//         },
//     };
//     println!("file opened successfully: {:?}", greeting_file);
// }

use std::{fs::File, println};

// fn main (){
//         let greeting_file = File::open("hello.txt").unwrap();
//  println!("file opened successfully: {:?}", greeting_file);
// }

// use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
