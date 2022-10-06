use std::{i8,i16,i32,i64,u8,u16,u32,u64,isize,usize,f32,f64};
use std::io::stdin;

fn main() {
    println!("Another rust");
    let num:i8 = 10;
    let name:&str = "Bek";
    let sur_name: &str = "Wakabore";
    println!("the number is {}", num);
    println!("my name is {} + {}", name, sur_name);
    /**
    *Multiple line comment
    */
    //single line comment

    let mut num2:i32 = 100;
    num2 = 3000;
    println!("the changed values is {}", num2)
}