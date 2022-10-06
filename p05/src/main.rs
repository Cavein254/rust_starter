fn main() {
//    let fruits = vec!["oranges", "mango", "pinapple"];

//    for(index, i) in fruits.iter().enumerate() {
//     println!("fruits number {} is {} ", index, i)
//    }

//    let mut num:u8 = 0;
//    while num <=20 {
//     println!("The number is {}", num);
//     num +=1;
//    }

    let x = 0..140;
    for i in x {
        if (i % 3 == 0) && (i % 6 == 0) && (i % 9 ==0){
            println!(" {} - ", i);
        }
    }

}
