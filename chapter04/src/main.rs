
//4.1 Owenership
// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); 
//     (s, length)
// }


// 4,2 Referencing and Borrowing

// fn main(){
//     let s=String::from("hello");

//     let len=calculatelength(&s);

//     println!("the length of {s} is {len}")
// }
// fn calculatelength(s:&String)->(usize){
// let len=s.len();
// len

// }


// //mutable refrences 
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }



//fixing unsafe program :copying vs moving out of a collection 

use std::{println};

// fn copying_moving_out(){

//     let vector: Vec<i32>=vec![0,1,2];
//     let n_ref : &i32=&vector[0];

//     let n: i32= *n_ref;
// println!("{n}");
// the above program woudld run where as the below program would not run why because 

// let v: Vec<String> = 
//   vec![String::from("Hello world")];
// let s_ref: &String = &v[0];
// let s: String = *s_ref;

// sting does have the copy trait 
// copying a String copies a pointer to heap data. Copying an i32 does not.


// so tpo copy vector of strings 
// let v:Vec<String>=vec![String::from("hello world")];
// let s_ref:&String= &v[0];
// println!("{s_ref}");

// or we can clone the data 
// let mut s: String = v[0].clone();
// s.push('!');
// println!("{s}");


// }

// fn mutating_diffeernt_fields (){

//     //one could not pass name to a function that takes as input a value of type (String, String).) 
//     //But name.1 still retains the W permission, so doing name.1.push_str(...) is a valid operation.
//     let mut name =(
//         String::from("rakesh"),
//         String::from("paulraj")
//     );

//     let first:&String=&name.0;

//     name.1.push_str("developer");

//     println!("{first} {}",name.1)      
// }


// fn get_first(name: &(String, String)) -> &String {
//     &name.0
// }

// fn mutating_diffeernt_fields_error() {
//     let mut name = (
//         String::from("Ferris"), 
//         String::from("Rustacean")
//     );
//     let first = get_first(&name);
//     name.1.push_str(", Esq.");// error occurs here we have already lost the write  and ownacces as we used sent the referrnce to the get_first function
//     println!("{first} {}", name.1);
// }


// fn different_datatype_safety() {
// let mut a = [0, 1, 2, 3];
// let (a_l, a_r) = a.split_at_mut(2);

// let x = &mut a_l[1];
// let y = &a_r[0];
// *x += *y;
// }

//  the slice type 
fn main (){

}