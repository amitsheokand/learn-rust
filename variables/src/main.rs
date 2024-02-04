fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    const TWO: u32 = 1 + 1;
    println!("The value of TWO is: {TWO}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of inner x is {x}");
    }

    println!("The value of x is: {x}");


    let spaces = "    ";
    let space = spaces.len();
    println!("The value of space is: {space}");

    let mut spaces = "    ";
    // spaces = spaces.len(); // error[E0308]: mismatched types,  expected `&str`, found `usize`
    let space = spaces.len();

    println!("The length of spaces is: {space}");


}
