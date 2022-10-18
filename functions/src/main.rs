fn main() {
    /*
    Functions! 
    Declared with fn
    snake_case (lower_lower_lower) is convention
    ORDER of functions is UNIMPORTANT (compiled code)
    */

    println!("Hello, world!");

    another_function(5, "meters");

    bind_to_expression();

    let v = five();
    println!("The value of v is: {v}");

    let p = plus_one(5);
    println!("The value of p is {p}");
}

// The parameters can be passed, but need to be defined in the function definition

fn another_function(x: i32, unit_label: &str) {
    println!("The Measurement is: {x} {unit_label}");
}

// Functions are also statements! But statements can't return values (w/o explicit returns), 
    // so you can't assign a statement to a variable `(let x = let y = 6)` is invalid\
// You need to bind instead to expressions
fn bind_to_expression() {
    let z = {
        let zz= 3;
        zz + 1
    };

    println!("The value of z is {z}")
}

// You can return a value with `->`
// * These are NOT NAMED
// * These must be typed!

fn five() -> i32 {
    5 //n.b. no semicolon because it is an EXPRESSION
}

fn plus_one(p: i32) -> i32 {
    p + 1
}

