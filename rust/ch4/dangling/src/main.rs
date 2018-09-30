// dangling pointer 메모리르 생성한곳이 아닌 사용하는 곳에서 해제 함으로써 해제 이후 참조 포인터가
// 오류나는 경우


fn main() {
    let reference_to_nothing = no_dangle();
}

/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // 스코프가 벗어 나는 시점에서 s가 메모리 해제가 됨
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // 소유권이 이전되므로 s는 해제되지 않음
