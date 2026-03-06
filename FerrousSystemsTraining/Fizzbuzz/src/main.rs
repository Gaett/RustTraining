fn fizzbuzz(i: i32) -> String {
    let div_by_3 = i % 3 == 0;
    let div_by_5 = i % 5 == 0;

    if div_by_3 && div_by_5 {
        String::from("FizzBuzz")
    } else if div_by_5 {
        String::from("Buzz")
    } else if div_by_3 {
        String::from("Fizz")
    } else {
        i.to_string()
    }
}

// I would have liked to do the matched version (I tried but failed to do it), still here's the correction's version
// fn fizzbuzz_match(i: u32) -> String {
//     let remainders = (i % 3, i % 5);

//     match remainders {
//         (0, 0) => format!("FizzBuzz"),
//         (0, _) => format!("Fizz"),
//         (_, 0) => format!("Buzz"),
//         (_, _) => format!("{}", i),
//     }
// }

fn loop_on_fizzbuzz() {
    for j in 0..=100 {
        println!("{} : {}", j, fizzbuzz(j));
    }
}

fn main() {
    loop_on_fizzbuzz();
}
