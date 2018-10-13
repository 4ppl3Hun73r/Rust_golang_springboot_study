fn main() {
    let mut s= String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2); // 인자에 대한 소유권을 가져가지 않음
    println!("s2 is {}", s2); // 정상 출력됨

    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);


    let s1 = String::from("hello");
    //  compile error let h = s1[0];

    // String wrapper Vec<u8>
    let len = String::from("hello").len();
    println!("{}", len);
    let len = String::from("Здравствуйте").len();
    println!("{}", len);

    let hello = "Здравствуйте";
    // compile error let answer = &hello[0];

    let s = &hello[0..4];
    println!("{}", s);
    // runtime panic let s2 = &hello[0..1];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
