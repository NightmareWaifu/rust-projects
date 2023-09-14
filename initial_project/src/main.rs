use std::io;


fn main() {
    println!("Program Start");
    separator("Learning Rust basics");
    
    
    //variables
    separator("Variables");
    let x = 5; //x is immutable
    println!("x value: {x}");


    let mut y = 6; //y is mutable
    println!("initial y value: {y}");

    y = 7; //update y value

    println!("updated y value: {y}");

    const THIS_IS_A_CONSTANT:i32 = 100; //constants are always immutable

    println!("Constant values do not change: {THIS_IS_A_CONSTANT}");

    let test_string = "how long am i?";
    let check_length = test_string.len();
    println!("\"{test_string}\" has a length of: {check_length}");
    
    //shadowing
    separator("Shadowing");

    let a = 5;
    println!("initial a value: {a}");

    let a = a + 1;
    println!("a value: {a} (using addition)");

    let a: i32 = a * 2;
    println!("a value: {a} (using multiplication)");

    let b = 10;
    println!("b value in outer scope: {b} (before)");

    {
        let b = b * 2;
        println!("b value in inner scope: {b}"); //temporarily changes the value of b
    }

    println!("b value in outer scope: {b} (after)"); //b goes back to its original value


    separator("Data types");

    let c:f64 = 10.5; //floating point number and "f32" data type also exists
    println!("x floating point value: {c}");

    let number_to_convert: &str = "100";
    let convert_to_int: u32  = number_to_convert.parse().expect("Not a number?"); //parse an int of type string into int
    println!("Converted string to int: {convert_to_int}");

    separator("Logic Statements");
    logic_statements();

    separator("Loops");
    loops();
}

fn separator(title:&str) {
    //pass in title to separate
    let separator = "-".repeat(10);
    println!("{separator}");
    println!("{title}");
    println!("{separator}");

}

fn logic_statements(){

    let number1:u32 = 50;
    println!("Number is {number1}");
    if number1 < 100{
        println!("Number is less than 100!");
    } else{
        println!("Number is more than 100!")
    }

    let condition = true;
    let if_true:u32 = 1;
    let if_false:u32 = 0;
    let number2:u32 = if condition { if_true } else { if_false }; //one liner if statement, both must be of same data type
    println!("{number2} is the result of an if statement in let! (condition: {condition})");

}

fn loops(){
    let mut counter = 0;
    loop{
        println!("In a loop!");

        if counter > 10{
            break counter = 0;
        }
        counter += 1;
    }

    separator("Same loop but in a let:");
    let result = loop{
        println!("In a loop! Part 2!");

        if counter > 10{
            let return_this = "Returned this value!";
            break return_this;
        }
        counter += 1;
    };

    println!("Result is: {result}");
    

    separator("Exit outer loop from inner loop using id");
    let mut count = 0;
    'loop_id: loop {
        //loops 2 times
        println!("count = {count}");
        let mut remaining = 0;

        loop {
            //loops 10 times 
            println!("remaining = {remaining}");
            if remaining == 10 {
                break;
            }
            if count == 2 {
                break 'loop_id; //breaks the outer loop -> to exit
            }
            remaining += 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    separator("While Loops");
    let mut while_number = 5;
    while while_number != 0{
        println!("{while_number}");

        while_number -= 1;
    }
    println!("Here we go!");

    separator("For loops");

    let my_list = [1,2,3,4,5];

    for i in my_list{
        println!("Looped through list: {i}");
    }
}

//stopped at 4. Understanding Ownership