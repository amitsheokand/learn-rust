// Title: Functions
fn main() {
    another_function();
    another_function_with_parameters(5);
    print_labeled_measurement(10, 'm');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let z = plus_one(5);
    println!("The value of x is: {}", z);

    println!(
        "{}",
        f({
            let y = 1;
            y + 1
        })
    );
}

fn another_function() {
    println!("Another function.");
    println!("This is a statement.");
}

fn another_function_with_parameters(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// quiz

fn f(x: i32) -> i32 {
    x + 1
}
