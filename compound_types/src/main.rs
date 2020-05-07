fn main() {

    // Declaring a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring a tuple
    let (_x, y, _z) = tup;
    println!("The value of of y is: {}",y );

    // Accesing tuple elements using `.` followed by index
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    println!("a = {}, b = {}, c = {}",a,b,c );


    // Declaring Arrays
    let arr = [1, 2, 3, 4, 5];
    println!("The array is {:?}",arr );
    // Type with number of elements
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The arr:[i32;5] contains{:?}", arr1);

    let tres = [3;5];
    println!("[3;5] will give {:?}",tres);
}
