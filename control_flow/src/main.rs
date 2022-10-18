fn main() {
    numlt5(7);
    nzero(3);
    numdivis(10);
    demo_let_if(true);
    let el = endless_loop("Again!");
    println!("The endless loop ran {el} times \n");
    label_loops();
    cond_loop();
    println!("\nWHILE vs FOR Loops");
    while_collection();
    for_collection();
    println!("\nCOUNTDOWN w/ FOR LOOP");
    for_countdown(7);
}

fn numlt5(x: i32) { 
    if x < 5 {
        //n.b. Rust won't convert anything to bool
        // so if number {... fails
        println!("The number is LT 5"); 
    } else {
        println!("The number is GTE 5");
    }
}

fn nzero(x: i32) {
    if x != 0 {
        println!("number was something other than zero");
    }
}

fn numdivis(x: i32) {
    if x % 4 == 0{
        println!("number is divisible by 4");
    } else if x % 3 == 0{
        println!("number is divisible by 3");
    } else if x % 2 == 0{
        println!("number is divisible by 2 (but not 4)");
    } else {
        println!("number is not dividisible by 4, 3, or 2");
    }
}

fn demo_let_if(x: bool) {
    let cond = if x { "Is True" } else { "Is False" };
    // n.b. ALL arms MUST pass the same type
    println!("The Condition: {cond}");
}

// Loops: 
    // loop repeats until told to stop
    // while loops until a condition is met
    // for does it the specified number of times

fn endless_loop(x: &str) -> i32 {
    let mut i = 0;
    loop{
        println!("{}", x);
        i += 1;
        if i >= 10{
            break i;
        }
    }
}

// Loop Labels!
    // You can define loop labels with a single quote ' and call continues and breaks from there

fn label_loops(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End Count = {count}");
}

fn cond_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

//  For loops, looping through a collection

// WHILE example is unsafe
fn while_collection() {
    println!("\nWhile Loop");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is {}", a[index]);
        index += 1;
    }
}

// For loop is much safer
fn for_collection() {
    println!("\nFor Loop");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

// FOR loops are the most used Rust loop
// For counting up/down you can use a range
fn for_countdown(x: i32) {
    for number in (1..(x+1)).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF !!!")
}