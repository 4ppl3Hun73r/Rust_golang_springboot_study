use std::fmt::Display;

/* life time compiler error
struct ImportantExcerpt {
    part: &str,
}
*/

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        // self 로의 참조자의 라이프타임은 명시할 필요가 없음 
        3
    }

    // 각 파라미터에 대하 lifetime이 컴파일러로 부터 부여됨.
    // 반환값이 self 에서 반환되면서 self의 lifetime을 가져감
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    } 
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {part: first_sentence};


    // lifetime이 생략가능한 패턴이 있음 lifetime elision rules
    // 라마미터에 대한 라이프타임 input lifetime, 반환값에 대한 라이프타임 output lifetime

    println!("{}", i.level());
    println!("{}", i.announce_and_return_part("str"));

    // static 은 프로그램 전체에 대한 lifetime을 가지게 됨
    let s: &'static str = "I have a static lifetime.";

    println!("{}", longest_with_an_announcement("str1", "str222", s));

}


fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


