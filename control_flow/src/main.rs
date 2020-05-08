fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //  Using If in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 }; // same value types

    println!("The value of number is: {}", number);


    // loop
    let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

    println!("The result is {}", result);


    // While loop
    let mut num = 3;

    while num != 0 {
        println!("{}!", num);
        num -= 1;
    }

    // For loop

    for ele in (1..4).rev() {
        println!("{}!",ele);
    }

    
    let arr: [i8;5]  = [10,20,30,40,50];

    for (i,ele) in arr.iter().enumerate(){
        println!("Element at index {} is {} ", i, ele);
    }
}
