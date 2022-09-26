fn main() {
    // One reason to use shadowing is to create some intention mutability (changes)
    // Without making the variable itself mutable (e.g. by forcing mutability with `let`)
    
    let x = 5;
    println!("The initial value of x is: {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The final value of x in the outer scope is: {x}");

    // type can also be changed with `let` but not with `mut`
    let spaces = "     ";
    let spaces = spaces.len();
    println!("Spaces, number of: {spaces}");

    // The below fails, per example
    // let mut spaces_mut = "     ";
    // spaces_mut = spaces_mut.len();
}
