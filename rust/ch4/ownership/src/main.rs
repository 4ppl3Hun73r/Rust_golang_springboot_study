fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello2");
    // String::from(s1) 도 소유권 이전으로 보나봄...
    // 그렇게 코드를 넣었더니 소유권 오류 발생

    let s3 = takes_and_gives_back(s2);

    // s2는 소유권을 줘버려서 사용 불가 println!("{}, {}, {}", s1, s2, s3);
    // s3가 s2의 소유권을 가지게 됨
    println!("{}, {}", s1, s3);

    // tuple을 이용해서 소유권과 함수 결과를 같이 받을 수 있음
    let (s2, len) = calculate_length(s1);
    // 소유권 반납작업이 너무 불필요하고 과해 보이네..

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}


fn _main2() {
    let s = String::from("hello");

    // function에 ownership을 넘겨줌.
    takes_ownership(s);

    // ownership을 넘겨준 후여서 사용 불가 println!("{}", s);

    let x = 5;

    makes_copy(x);

    // i32형은 복사로 넘어가게 되므로 사용 가능
    println!("{}", x);

    // x 에 대한 해제는 여기에서 진행됨
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);

    // s에 대한 메모리 해제는 여기에서 진행됨
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);

    // some_integer는 x의 복사값이며 해제는 여기에서 진행됨
}

fn _main1() {
    let s = "Hello";

    println!("{}, world!", s);

    let mut s = String::from("Hello");
    s.push_str(" World!");

    println!("{}", s);

    // working
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    // working
    let s1 = "Str";
    let s2 = s1;
    println!("{}, {}", s1, s2);

    /* compile error
    let s1 = String::from("Str");
    let s2 = s1;
    println!("{}, {}", s1, s2);
    */

    // working
    let s1 = String::from("Str");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    // Copy Trait 를 가지고 있으면 clone같이 안해도 복사가 됨
    // 정수형이나 bool, 소수점 타입, Copy 가능 타입으로 구성된 튜플 들 같은 형이 대표적인 예

}
