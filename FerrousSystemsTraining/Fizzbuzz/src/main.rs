fn fizzbuzz(i: u32) -> String {
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

fn fizzbuzz_match(i: u32) -> String {
    let remainders = (i % 3, i % 5);

    match remainders {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        (_, _) => i.to_string(),
    }
}

fn loop_on_fizzbuzz() {
    for j in 0..=100 {
        println!("{} : {}", j, fizzbuzz(j));
        println!("{} : {}", j, fizzbuzz_match(j));
    }
}

fn main() {
    loop_on_fizzbuzz();
}
