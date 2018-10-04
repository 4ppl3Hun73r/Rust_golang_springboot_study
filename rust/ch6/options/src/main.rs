// Option<T>


fn main() {
    // Some 어떤 타입의 데이터라도 가질 수 있다는 것을 의미함
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // <>로 타입 추론을 하게 해줘야함

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is not satisfied
    // let sum = x + y;
}
