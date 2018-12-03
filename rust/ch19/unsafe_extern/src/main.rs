extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        // 타 언어를 호출할때는 타 언어는 러스트의 규칙을 보장 할 수 없기 때문에
        // unsafe로 감싸야함
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}