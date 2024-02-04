use std::io;

fn main() {

    let guess: u32 = "42".parse().expect("Not a number!");

    /// scalar types are
    // integer
    // floating-point number
    // boolean
    // characters
    ///

    // integer types

    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Number literals	Example
    // Decimal	        98_222
    // Hex	            0xff
    // Octal	        0o77
    // Binary	        0b1111_0000
    // Byte (u8 only)	b'A'

    // integer types default to i32
    // isize or usize is when indexing collection.

    // floating-point types
    // default type is f64 because on modern CPUs

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // numeric operations
    let sum = 5 + 10; // 15 i32
    let difference = 95.5 - 4.3; // 91.2 f64
    let product = 4 * 30; // 120 i32
    let quotient = 56.7 / 32.2; // 1.7608695652173913 f64
    let truncated = -5 / 3; // -1 i32
    let remainder = 43 % 5; // 3 i32

    // boolean type
    let t = true;
    let f: bool = false;

    // character type
    // char type is four bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z : char = 'ℤ';
    let emoji = '🤪';


    let mut x :u8 = 0;
    // println!("x -1 : {}", x-1); // underflow, error in debug mode


    /// compound types are
    // tuple
    // array
    ///

    // tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("tuple values are : {}, {}, {}", tup.0, tup.1, tup.2);

    // destructuring a tuple
    let (x, y, z) = tup;
    println!("x, y, z : {}, {}, {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred, six_point_four, one : {}, {}, {}", five_hundred, six_point_four, one);

    // unit tuple, without any values
    let unit_tup = ();

    // array
    let a = [1, 2, 3, 4, 5]; // array of 5 elements of type i32 [i32, 5]

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"]; // [&str; 12]

    let array_with_type_annotation: [i32; 5] = [1, 2, 3, 4, 5]; // with type annotation

    let array_with_default_values = [3; 5]; // [3, 3, 3, 3, 3] // with initial value 3 and length 5


    // accessing array elements

    let first = a[0];
    let second = a[1];

    // Invalid array access

    let new_array = [1, 2, 3, 4, 5];

    println!("Enter array index : ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.
        trim().
        parse()
        .expect("index entered was not a number");

    let element = new_array[index];

    // if index is out of range, program will panic and exit
    // eg

    // Enter array index :
    // 7
    // thread 'main' panicked at src/main.rs:119:19:
    //     index out of bounds: the len is 5 but the index is 7
    println!("The value of the element at index {} is: {}", index, element);


    // quiz

    // does not compile

    // let message = "The temperature today is:";
    // let x = [message, 100];
    // println!("{} {}", x[0], x[1]);

    // error[E0308]: mismatched types
    //    --> src/main.rs:133:23
    //     |
    // 133 |     let x = [message, 100];
    //     |                       ^^^ expected `&str`, found integer


    let message = "The temperature today is:";
    let x = [message; 100]; // replace , with ;,now initializes array with 100 elements of message
    println!("{} {}", x[0], x[1]);

    // compiles
    let t = ([1; 2], [3; 4]);
    println!("{:?}", t);
    let (a, b) = t;
    println!("{}", a[0]+ t.1[0]);

    // result : ([1, 1], [3, 3, 3, 3])
    // 4




}
