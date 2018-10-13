use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // compile error value moved map, which does not implement the `Copy` trait println!("{}, {}", field_name, field_value);

    println!("{:?}", map);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // return Option
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    scores.entry(String::from("Blue"));
    //let what = scores.entry(String::from("Blue"));
    //println!("{:?}", what); // Entry(OccupiedEntry { key: "Blue", value: 10 })
    // what 이 수정권한을 빌려가버려서 반납하지 않고는 score 에서 빌릴수 없음
    //what.or_insert(50); // 아마도 수정권한이 반납 됨?
    // 안되네... 머지?
    let what = scores.entry(String::from("Red"));
    println!("{:?}", what); // Entry(VacantEntry("Red"))
    // entry 앞을 let xx 로 받으면 map의 수정권한을 넘어가는 걸로 보임.
    // 흠.. 그럴경우에 반납은 어떻게 하면 되지???

    let text = "hellow world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
