fn main(){
    let a = 10;
    let b = 20;
    let result = get_ref(&num1,&num2);
    println!("{}",result);

}

fn get_ref<'a,'b>(param_1: &'a i32, param_2: &'b i32) -> &'a i32{
    println!("{}",param_2);
    param_1
}