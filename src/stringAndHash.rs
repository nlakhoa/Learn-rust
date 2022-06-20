
// use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;


fn main(){
    let s3 = String::from("Hello everyone");
    // Bytes
    for i in s3.bytes(){
        println!("{}",i);
    }
    // Scalar values
    for i in s3.chars(){
        print!("{}",i);
    }

    let text = "hello world this is wonderfull world";
    let mut map = HashMap::new();
    for i in text.split_whitespace(){
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    print!("{:?}",map);
}   