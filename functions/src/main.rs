fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 6;
    println!("Calling another_function with value, x = {} & y = {}", x, y);
    another_function(x, y);

    // Expression and statement
    let _a = 5; // Statement

    let b = {
        let a = 15;
        a + 10 // Expression
    };
    println!("The value of b = {}",b); // 25

    println!("Calling function with return value of {} ",five() );

    let value = one_plus(6);
    println!("The value of one_plus(6) is {}", value);
}


fn another_function(x: u8, y: u8){
    println!("We are in another_function with value, x = {} & y = {}", x, y);
}

// Fumction returns values
fn five() -> i8{
    5
}

fn one_plus(n: i8) -> i8{
    n + 1
}
