fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    println!("{}", word);

    // compile error s.clear();

    let mut s = String::from("hello world");
    let word = first_word2(&s);
    println!("{}", word);


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    // slice로 자른거는 compile 단계에서 길이 체크를 못해줌.. 당여한 건가?
    println!("{}", slice[4]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn _main() {
    let mut s = String::from("Hello, world!");

    let word = _first_word(&s);

    println!("{}", word);

    s.clear(); // s값을 "" 로 변경

    // s 는 "" 이지만 word는 여전이 6의 값을 가지고 있음
    println!("{}", word);

    // 버그 유발 코드

    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}, {}, {}", s, hello, world);
}


// 문장에서 첫번째 단어를 잘라서 해당 단어를 반환
fn _first_word(s: &String) -> usize {

    // byte 배열로 변경
    let bytes = s.as_bytes();

    // 배열에서 iterator 생성
    // enumerate 는 먼지 잘 모르겠다. iter 의 결과를 튜블로 반환한다 정도..
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
