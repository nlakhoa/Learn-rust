fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_ownership(s2);
    println!("{},{}", s1, s3);
}

fn gives_ownership() -> String {
    let some_ownership = String::from("Hello");
    some_ownership
}

fn takes_ownership(some_string: String) -> String {
    some_string
}
