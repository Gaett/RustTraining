fn test_vectors() {
    let _v: Vec<i32> = Vec::new(); // dynamic array that can grow / shrink
    let _v2: Vec<i32> = vec![1, 2, 3];

    let mut v3: Vec<i32> = vec![1, 2, 3];
    v3.push(5);

    println!("{:?}", v3);

    let third: &i32 = &v3[2]; //Direct indexing from vector :o
    println!("third element {}", third);

    let second = v3.get(1);
    match second {
        Some(x) => println!("{}", x),
        None => println!("Failed"),
    }
}

fn test_utf8() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used due to fn add(self, s: &str) -> String {

    println!("s3 is {s3}");
}

fn test_hash_maps() {
    use std::collections::HashMap; //Local scope of use ...

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); //No need for option match cause of default value

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //Checking if value is a new entry or will be modified
    let key = String::from("Blue");

    let old_value = scores.get(&key).cloned(); // Sauvegarde la valeur existante
    let new_value = scores.entry(key).or_insert(50);

    if old_value.is_none() {
        println!("New entry with value {}", new_value);
    } else {
        println!("Existing entry with updated value {}", new_value);
    }

    scores.entry(String::from("Blue")).or_insert(50); //returns mut ref to value (existing or newly inserted)

    // Ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    for (key, value) in &map {
        println!("{key}: {value}");
    }

    // println!("{}", field_name); Value is borrowed and not given back

    let field_name = String::from("Favorite color plus !"); //But with some shadowing we can reuse the name (losing the access to the previous field_name in case the map gives back the ownership)

    println!("{}", field_name);

    for (key, value) in &map {
        println!("{key}: {value}");
    }
}

fn main() {
    test_vectors();
    test_utf8();
    test_hash_maps();
    // let mut scores = HashMap::new(); Doesn't work because of local scope of use
}
