fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

/* 크기를 알수 없어서 Box로 감싸야함
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
*/

fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}