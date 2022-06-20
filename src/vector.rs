fn main() {
    let a = [1, 2, 3];
    let v = vec![1, 2, 3];
    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);
    println!("{:?}", v2);

    enum SheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SheetCell::Int(12),
        SheetCell::Float(10.12),
        SheetCell::Text(String::from("Yi long ma"))
    ];

    match &row[2] {
        &SheetCell::Float(i)=>println!("{}",    i),
        _ => println!("this is not a float")
    }
}
