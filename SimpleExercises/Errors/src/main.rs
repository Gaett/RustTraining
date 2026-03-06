// enum Option<T> {
//     //define the generic option type value // no value
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     // generic result type value or error
//     Ok(T),
//     Err(E),
// }

fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by 0"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let res = divide_option(10.0, 0.0);
    match res {
        Some(x) => println!("Result : {}", x),
        None => println!("Cannont divde by Zero"),
    }
    let res = divide_option(10.0, 1.0);
    match res {
        Some(x) => println!("Result : {}", x),
        None => println!("Cannont divde by Zero"),
    }

    let res_result = divide_result(10.0, 0.0);
    match res_result {
        Ok(result) => println!("Result : {}", result),
        Err(err) => println!("Error : {}", err),
    }

    let res_result = divide_result(10.0, 1.0);
    match res_result {
        Ok(result) => println!("Result : {}", result),
        Err(err) => println!("Error : {}", err),
    }
}
