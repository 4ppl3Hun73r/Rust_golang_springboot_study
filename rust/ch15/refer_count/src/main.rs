/*
enum List {
    Cons(i32, Box<List>),
    Nil,
}
*/

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5,
        Rc::new(Cons(10,
            Rc::new(Nil)))));
    // let b = Cons(3, Box::new(a));
    // compiler error -> b가 a를 가져갔기 때문에 
    // let c = Cons(4, Box::new(a));

    // rc clone을 이용해서 레퍼런스만 복사하고 카운트를 증가 시킴
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // 참조 카운트 확인하기

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        // scope를 벗어나면서 자동으로 카운트가 감소함
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}