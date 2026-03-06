// Ownership, borrowing and references
// Only one owner at the time

// each value has an owner (1)
// Only one owner
// Owner out of scope => value dropped

fn owernship() {
    let s1: String = String::from("RUST");
    let len = calculate_lengths(&s1); //borrowing s1 ?
    println!("Length of s1 : '{}' is {}", s1, len);

    let s2: String = s1;
    println!("{}", s2); //Taking ownership away

    // s1 and s2 stop existing oustide main()
    /*s: &String, reference to the String */
    fn calculate_lengths(s: &String) -> usize {
        s.len()
    }
}

fn borrowing() {
    let _x: i32 = 5;
    let _r: &i32 = &_x; //reference to the original owner
    //     *_r += 1; cannot borrow `*_r` as mutable, as it is behind a `&` reference `_r` is a `&` reference, so it cannot be borrowed as mutable
    println!("Value of _x : {}", _x);
    println!("Value of _r : {}", _r);

    let mut _y: i32 = 5;
    let _s: &mut i32 = &mut _y; //reference to the original owner, modifiable
    // println!("Value of _y : {}", _y); => can't because is borrowed
    *_s += 1;
    println!("Value of _s : {}", _s);
    println!("Value of _y : {}", _y); // now is fine cause we probably gave back the ownership, wouldn't compile if we used *_s after this line
}

fn immutable() {
    struct BankAccount {
        owner: String,
        balance: f64,
    }

    impl BankAccount {
        /* withdraw is mutable for the struct */
        fn withdraw(&mut self, amount: f64) {
            println!(
                "Withdrawing {} from account onwed by {}",
                amount, self.owner
            );
            self.balance -= amount;
        }

        /* check_balance is immutable for the struct */
        fn check_balance(&self) {
            println!(
                "Account owned by {} has a balance of {}",
                self.owner, self.balance
            );
        }
    }

    let mut account: BankAccount = BankAccount {
        owner: String::from("Alice"),
        balance: 6.4,
    };
    //immutable access
    account.check_balance();

    //mutable borrow
    account.withdraw(64.);

    account.check_balance();
}

fn variable_mutability() {
    let _a: i32 = 12; //underscore cause not used
    // a += 1; won't work cause immutable by default

    let mut _b: i32 = 12; //underscore cause not used
    _b += 1;
}

fn constants() {
    let mut _x: i32 = 5; //should not leave mut is we don't modify it
    // const mut y: i32 = 10; const is immutable !
    const _Y: i32 = 10; //Capital letter for constants
    println!("{}", _x);
    println!("{}", _Y);

    const NOT_PI: f64 = 3.14111111111;
}

fn main() {
    owernship();
    borrowing();
    immutable();
    variable_mutability();
    constants();
}
