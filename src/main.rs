fn main() {
    println!("Hello, world!");

    log_function();

    let x = sum_of_numbers(12, 6);
    println!("return functions {x}");

    let return_values = is_even(x);
    println!("Is sum of the numbers is even {return_values}");
}

// function not returning any thing just changes is ->
fn log_function() {
    println!("Functions test one!");
}

//return functions
fn sum_of_numbers(x: i32, y: i32) -> i32 {
    let add: i32 = x + y;
    println!("sum of {x} and {y} is {add}");
    if x > 10 {
        println!("if statement result {x}");
    }
    return x + y; //1: method to return values from functions
                  // x+y  // 2: method to return values from functions
}

fn is_even(number: i32) -> bool {
    if number % 2 == 0 {
        // example of early return
        return true;
    } else {
        false
    }
}
