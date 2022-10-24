fn main() {
    // Rust is statically typed and MUST know variable typing at run time
    // Two main types: scalar (single value) and compound
    
    // Scalars
    /*
    Signed numbers are stored using two's complement
    You can use an underscore `_` to make numbers readable at the major delimiters (e.g. 1_000)
    isize and usize are based on the computer architecture
    Integers will overflow if in --release mode
    Defaults are f64 and i32
    You get a floor if you divide two ints
    Char is a four bit unicode scalar
    */

    // Compound Types
    /* 
    Multiple types in one group (collection?)
    Tuples and arrays
    */

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // This pattern is called *destructuring* (setting individual variables to tuple values)
    let (_x, y, _z) = tup;

    // One can also access the elements of a tuple with dot (.) notation
    let five_hundred = tup.0;
    let one = tup.2;

    println!("The value of Five-Hundred is {five_hundred}");
    println!("The value of y is: {y}");
    println!("The value of one is: {one}");

    /* 
    The Array Types: Use when you want a stack instead of a heap 
    OR want a fixed number of elements.
    n.b. A vector is similar but is not fixed in size
    An Array is best when you know the number of elements will be fixed.
    e.x. include: Months
    */

    // Call an array with assignmentL [type; size] = [n1, n2, ... nn]
    let a: [i32; 5] = [1,2,3,4,5];

    // You can (re)assign all 5 elements to 3 [assignment; length]
    let a = [3; 5];

    // Howto Access Array Elements
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];

    println!("First Element is {first}, Second Element is {second}");
    // Rust's memory safety means accessing an element out of range will PANIC the kernel
    // This is a FEATURE not a BUG

}
