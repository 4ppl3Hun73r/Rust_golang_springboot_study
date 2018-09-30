fn main() {
    // '&' References 를 사용하여 소유권을 넘겨주는게 아니라 '빌려' 줄수 있음
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    //change(&s1);
    // 인자가 가변 참조자이기 때문에 해당 함수를 사용하려면 가변으로 생성 해야함
    let mut s = String::from("hello");
    println!("{}", s);
    change(&mut s);
    println!("{}", s);

    // 가변 참조자는 1번 이상 빌려줄수 없음
    // let r1 = &mut s;
    // let r2 = &mut s;
    {
        let r1 = &mut s;
        // 스코프로 빌려줄수는 있음
    }
    let r2 = &mut s;

    mutable_test();

}

fn mutable_test() {
    let mut s = String::from("hello");

    let r1 = &s; // 문제 없음
    let r2 = &s; // 문제 없음
    //let r3 = &mut s; // 문제 발생
    // 빌려갔을떄 값이 변경되면 안되기 떄문에 빌려간 상태에서 값 변경가능한 빌림을 못함
    // 동일하게 순서가 변경되도 오류가 발생

}

// 빌려왔지만 원본이 불변이기 떄문에 수정 불가
//fn change(s: &String) {
fn change(s: &mut String) { // 가변 참조자로 변경
    s.push_str(", world");
}

// 소유권을 빌려감.
fn calculate_length(s: &String) -> usize {
    s.len()
}
// 소유권을 반납하기 때문에 메모리 반납은 이루어 지지 않음
