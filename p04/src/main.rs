fn main() {
    let mut n = 0;
    loop {
        n = n+1;
        if n == 13 {
            println!("printing {}", n);
            continue;
        }
        if n == 15 {
            break;
        }
        println!("The value of n is {}", n);
    }
}
