fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    /* compile error. lifetime!
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */
}

//    &i32      // a reference
//    &'a i32   // a reference with an explicit lifetime
//    &'a mut i32   // a mutable referece with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/* x를 참조할지 y를 참조할지 함수에서 알수 없음
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

fn _main2() {

    /*
    let r;
    {
        let x = 5;
        r = &x;
    }  // compile error , lifetime!!
    */

    /* oompile error, lifetime. r의 lifetime길이가x보다 크기 때문에 안됨
    let r;
    let x = 5;
    r = &x;
    */

    let x = 5;
    let r = &x;

    println!("r: {}", r);
}
