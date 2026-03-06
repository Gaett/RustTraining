fn main() {
    function_after_main();
    function_after_main_parameter(-32);
    expressions();
    // expressions; => can't call without the parenthesis

    let _x: i32 = {
        //would be _X outside the main as a const
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty //same as return price * qty
    };
    println!("result is {}", _x);
    println!("Hello, world!");

    let weight: f64 = 70.0;
    let height: f64 = 1.82; //meters
    let bmi: f64 = calculate_bmi(weight, height);
    let bmifn: fn(f64, f64) -> f64 = calculate_bmi; //function alias => not sure if recommanded in rust ?
    println!("your bmi is {} or {}", bmi, bmifn(weight, height));
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight * height
}

fn function_after_main() -> i32 {
    1 // return + ; not mandatory 
}

fn function_after_main_parameter(height: i32) -> i32 {
    height
}

fn expressions() {
    //loops
    let mut i = 0;
    loop {
        i += 1;
        if i > 100 {
            break;
        }
    }
    println!("{}", i);

    //loops as blocks
    let mut i = 0;
    let loop_result = loop {
        i += 1;
        if i > 10 {
            break 6;
        }
        println!("i = {}", i);
    };
    println!("loop_result = {}", loop_result);

    //while
    let mut i = 0;
    while i < 10 {
        i += 1;
        println!("i = {}", i);
    }

    //for loops
    for num in 0..10 {
        println!("{}", num);
    }

    //for loops + iterable
    for ch in "Hello".chars() {
        //Which is chars().into_iter() loop on match iter.next...
        println!("{}", ch);
    }

    //matches
    let a = 4;
    match a % 3 {
        0 => {
            println!("{} divisible by 3", a)
        }
        _ => {
            // _ placeholder for anything else ?
            println!("{} not divisible by 3", a)
        }
    }
}
