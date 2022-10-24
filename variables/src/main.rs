fn main() {
    
    // Running the below w/o the `mut` throws a compile error 
    // because x is instantiated as immutable (by default)
    let mut x = 5;
    println!("The value of x is {x}");
    
    x = 6;
    println!("The value of x is {x}");

    // n.b. constants are ALWAYS immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("Three Hours in Seconds is: {THREE_HOURS_IN_SECONDS} seconds");

}
