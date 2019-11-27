use std::collections::HashMap;

fn main() {
    test_creating_hashmaps();
    test_accessing_hashmap();
    test_iterating_hashmap();
    test_updating_hashmap();
}

fn test_creating_hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // We can also zip tuples together to form a hashmap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // Keys and values are owned by the hashmap once inserted, so:
    let key = String::from("Green");
    let mut another_map = HashMap::new();
    another_map.insert(key, 40);

    // It's illegal to use key here.
    //println!("Key is {}", key);
}

fn test_accessing_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        None => println!("{} team has no score", team_name),
        Some(score) => println!("{} team scored {}", team_name, score)
    }    
}

fn test_iterating_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn test_updating_hashmap() {
    // Sorry for the repetition :/
    // BTW, If you're not me and you're reading this, why? :P
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Overwriting:
    scores.insert(String::from("Blue"), 75);
    println!("{:?}", scores);

    // Using the entry api to only insert if there's no existing value
    scores.entry(String::from("Pink")).or_insert(50);
    // Yellow will remain 50 because it's already in the map
    scores.entry(String::from("Blue")).or_insert(12);
    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello there world you wonderful world that you are";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
