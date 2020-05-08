fn main() {
    let f: u32 = fibonacci(5);
    println!("The 5th number in fibonacci sequence is: {}", f);
}

fn fibonacci(n: u32) -> u32{
    if n == 0  {
        0
    }else if n == 1 || n == 2{
        1
    }
    else{
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
