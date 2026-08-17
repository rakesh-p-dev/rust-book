use std::io;
fn main(){
 let s=String::from("hello");
 let (s2,len)=get_length(s);


 println!("The length of '{s2}' is {len}.");

}


 fn get_length(s:String)-> (String,usize){
let length=s.len();
 (s,length)
 }




