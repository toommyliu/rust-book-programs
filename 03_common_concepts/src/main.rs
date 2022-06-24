fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut counter = 5;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }

    let fahrenheit = celsius_to_fahrenheit(100);
    println!("{}F", fahrenheit);

    let celsius = fahrenheit_to_celsius(50);
    println!("{}C", celsius);
}

fn another_function(x: i32) {
    println!("Hello from another function: {}", x);
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn celsius_to_fahrenheit(x: i32) -> i32 {
    (x * 9 / 5) + 32
}

fn fahrenheit_to_celsius(x: i32) -> i32 {
    (x - 32) * 5 / 9
}
