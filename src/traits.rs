struct Data {
    num1: i32,
    num2: i32,
    str1: String,
    optional: Option<i32>,
}
struct Data2 {
    num1: i32,
    num2: i32,
    str1: String,
    optional: Option<i32>,
}

impl Data {
    fn new() -> Self {
        Data {
            num1: 12,
            num2: 24,
            str1: "Some string 2".to_string(),
            optional: None,
        }
    }
}



trait Transform {
    fn revert(&self) -> String {
        String::from("No String...")
    }
}
 
impl Transform for Data {
    fn revert(&self) -> String {
        self.str1.chars().rev().collect::<String>()
    }
}

fn main() {
    let a = Data::new();
    let b = Data2 {
        num1 : 10,
        num2: 20,
        str1: String::from("Hello"),
        optional: None
    };

    println!("{}", a.revert());
    println!("{}", b.revert());
}
