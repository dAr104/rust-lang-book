fn main() {
    let number = 5;

    if number < 10 {
        println!("The number is less than 10");
    } else if number > 22 {
        println!("The number is greater than 22");
    } else {
        println!("The number is 10 or greater and 22 or less");
    }

    let condition = true;
    let number = if condition {5} else {6};

    let mut counter = 0;

    let result = loop {
        println!("again!");
        counter += 1;
        if counter == 5 {
            break counter; // break out of the loop and return the value
        }
    };

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element)
    }

    for num in 1..4 {
        println!("{}!", num);
    }
}