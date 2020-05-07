fn main() {
    let x = 5;
    println!("The value of x is : {}",x);

    /*
    *Cannot change immutable variable x
    *x = 6;
    *println!("The value of x is : {}",x);
    */


    //  Add `mut` keyword to make a variable mutable
    let mut a = 5;
    println!("The value of a is: {}",a );

    a = 6;
    println!("The value of a is: {}",a );

    //  Decalaring a constant, type of the value must be annoted
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}",MAX_POINTS);

    //  Shadowing
    //  Can change type of variable
    let spaces = "    ";
    println!("spaces = {}",spaces );
    let spaces =  spaces.len();
    println!("spaces = {}",spaces );
    /*
    *Cannot change type of variable
    *let mut spaces = "    "
    *spaces = spaces.len();
    */

    // Floating type
    let f_x = 2.0; // f64
    println!("Value of float x: f64(default) = {}",f_x);
    let f_y: f32 = 3.0; // f32
    println!("Value of float y: f32 = {}",f_y);

    //  Airthmetic Operations
    // addition
    let sum = 5 + 10;
    println!("Addition {}",sum );
    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference {}",difference );
    // multiplication
    let product = 4 * 30;
    println!("Product {}",product );
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient {}",quotient );
    // remainder
    let remainder = 43 % 5;
    println!("Remainder {}",remainder );



    //  Boolean Types
    let t = true;
    println!("t is {}",t );
    let f: bool = false;
    println!("f is {}",f );

    //  Character Types
    let c = 'z';
    println!("c = {}",c );
    let z = 'ℤ';
    println!("z = {}",z );
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat = {}",heart_eyed_cat );

}
