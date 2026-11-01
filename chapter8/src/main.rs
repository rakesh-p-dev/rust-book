use std::collections::HashMap;

fn main() {

    let mut hashmap = HashMap::new();
    hashmap.insert("one",1);


    hashmap.entry("hundred").or_insert(100);
    hashmap.entry("one").or_insert(10);

    println!("{:?}",hashmap);
}


