#![allow(unused)]
use std::{i8,i16,i32,i64,u8,u16,u32,u64,isize,usize,f32,f64};
use std::io::{Write, BufReader, BufRead, ErrorKind };
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    enum Days{
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _=>false
            }
        }
    }

    let today::Day = Day::Monday;
}

// fn main() {
//     println!("Another rust");
//     let num:i8 = 10;
//     let name:&str = "Bek";
//     let sur_name: &str = "Wakabore";
//     println!("the number is {}", num);
//     println!("my name is {} + {}", name, sur_name);
//     /**
//     *Multiple line comment
//     */
//     //single line comment

//     let mut num2:i32 = 100;
//     num2 = 3000;
//     println!("the changed values is {}", num2)
// }

// fn main() {
//     // let st3 = String::from("x x x c d e e t y u i o p");
//     // let mut v1:Vec<char> = st3.chars().collect();
//     // v1.sort();
//     // v1.dedup();
//     // for c in v1 {
//     //     println!("{}", c);
//     // }
//     let st4:&str = "Random String";
//     let mut st5: String = st4.to_string();
//     let st6:&str = &st5[0..6];
//     println!("string st6 {}", st6);
//     st5.clear();
//     println!("string st6  clear {}", st5);
// }