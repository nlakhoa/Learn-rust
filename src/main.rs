mod struct;

fn main() {
    let number = [0, 1];
    let get_number = number[1];
    print!("{}", get_number);
    let _hashing = [0; 32];
    println!("{:?}", _hashing);
    for i in _hashing.iter() {
        print!("{} ", i);
    }
}
