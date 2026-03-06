use std::f64::consts;

//Simple types

fn test_ints() {
    let x: i32 = -42;
    let y: u64 = 100;
    // let y: u64 = -100; -> compiling error yay
    println!("Signed ints : {}", x);
    println!("Unsigned ints : {}", y);

    // difference between i32 and i64
    // i32: 2^31 - 1
    // i64: 2^63 - 1
    let _a: i32 = i32::MAX;
    let _b: i64 = i64::MAX;
    // println!("Diff : {}", a - b);
    // println!("Diff : {}", b - a);
    // No operator for i32 - i64 or i64 - i32
}

/* approximate value of `f{32, 64}::consts::PI` found
consider using the constant directly
for further information visit https://rust-lang.github.io/rust-clippy/rust-1.93.0/index.html#approx_constant
`#[deny(clippy::approx_constant)]` on by defaultclippyClick for full compiler diagnostic
f64
value of literal: 3.14 (bits: 0x40091EB851EB851F) */
fn test_floats() {
    let x: f32 = 0.; // needs the 0. instead of 0
    let y: f64 = 0.;
    let pi: f64 = 3.14; // using . not ,
    let pi2: f64 = consts::PI;
    println!("{} {} {} {}", x, y, pi, pi2); // println needs ! cause it's a macro, can't print stuff that aren't initialized
}

fn test_bool() {
    let is_snowing: bool = true;
    println!("Is it snowing ? {}", is_snowing);
}

fn test_char() {
    // Character Type - char : a single unicode value
    let letter: char = 'a';
    println!("First letter : {}", letter);
}

fn afunction(param1: i32) -> i32 {
    param1 + 1;
    return param1; //param1 not modified
}

//Compound types
// arrays, tuples, slices, strings (slice string)

fn arrays() {
    //homogeneous type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; //type; size
    // println!("{}", numbers); => doesn't work
    println!("numbers : {:?}", numbers); // works
    // dbg!("{}", numbers); //works as well
    // :? => print through debug
    // :#? => debug with indent
    // empty => classic display

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("fruits : {:?}", fruits); // works
    println!("fruits : {:?}", fruits[0]); // works
}

fn tuples() {
    //heterogeneous type
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    let human2: (&str, i32, bool) = ("Alice", 30, false);
    println!("Hooman : {:?}", human);
    println!("Hooman : {:?}", human2);

    // String : dynamically allocated ? heap
    // &str immutable, stack
}

fn slices() {
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("{:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("{:?}", animal_slices);

    let book_slices: &[&String] = &[&"LionKing".to_string(), &"AnotherOne".to_string()];
    println!("{:?}", book_slices);
}

fn string_vs_strslices() {
    // String : growable, mutable and owned string type; slow to access
    let a: String = "a".to_string();
    let mut a2: String = String::from("a2");
    a2.push_str(" something");

    // &str : string to copy, quicker
    let b: &str = "b";
    let c: String = b.to_owned(); // cloning

    let d: &str = &a2[0..5];

    println!("{} {} {} {} {}", a, a2, b, c, d);
}

fn shadowing() {
    /* Shadowing is different than mut */
    let x: i32 = 5;
    println!("before shadowing {}", x);
    let x: i32 = x + 1;
    println!("after shadowing {}", x);

    {
        //Innerblock
        let x: i32 = x * 2; //5+1 * 2
        println!("after shadowing {}", x);
    }
}

/// This is a comment block in Rust
fn main() {
    test_ints();
    test_floats();
    test_bool();
    test_char();
    println!("{}", afunction(32));
    arrays();
    tuples();
    slices();
    string_vs_strslices();
    shadowing();
}
