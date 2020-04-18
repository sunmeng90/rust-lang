fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("Team {} has score {}", team_name, s),
        None => println!(" Team {} has no score", team_name),
    }
    // println!("{:#?}", score);

    // iterate over a map
    for (key, value) in &scores {
        println!("{} -> {}", key, value);
    }
}
