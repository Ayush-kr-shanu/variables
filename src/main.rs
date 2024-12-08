fn main() {
    // variables in rust is immutable by default but can be changed with keyword mut before varibale name
    // mut keyword change the immutable variables to mutable variables
    let mut x = 10;
    println!("Value of X: {x}");

    x = 20;
    println!("New value of X: {x}");

    // const variable naming convention is when using const keyword variable name should be in upper case only with data type
    // const variable is immutable in any case it cant be change to mutable variable
    const HOUR_IN_SECOND: u32 = 60 * 60;
    println!("One hour in seconds: {HOUR_IN_SECOND}");

    // Shadowing the variable
    // When we declare a variable with let keyword and assign any value to it then again we declare the variable with same name using let keyword with any other value that is shadowing the variable
    let y = 10;
    println!("Value of Y: {y}");

    let y = "Hello";
    println!("New value of Y: {y}");

    // second example of shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");
}
