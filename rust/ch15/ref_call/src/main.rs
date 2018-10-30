#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 가변 참조자를 여러 곳에서 안전하게 참조 할 수 있는가?
    // 단일 스레드에서는 사용을 할 수 있다. 다중 스레드 환경에서는 보장 할 수 없다

    //Rc<T>는 동일한 데이터를 복수 수유 가능
    //Box<T>, RefCell<T>는 단일 소유자만 가능
    //Box<T>는 컴파일 타임에 검사된 불변 혹은 가변 빌림 허용
    //Rc<T>는 오직 컴파일 타임에 검사된 불변 빌림만 허용
    //RefCell<T>는 런타임에 검사된 불변 혹은 가변 빌림을 허용
    //RefCell<T>이 런타임에 검사된 가변 빌림을 허용하기 때문에, RefCell<T>이 불변일 때라도 RefCell<T> 내부의 값을 변경할 수 있다.

    let x = 5;
    // compile 에러 let y = &mut y;

    //RefCell은 컴파일은 넘어가도 런타임떄 검사를 하므로 panic!이 발생하게 된다

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // 너무 어렵다.. 복잡해.
    // 다만 알수 있는 한가지는 compile시점의 보수적인 부분을 런타임으로 옮김으로써
    // 내부 가변성을 확보 할 수 있다는 것이다. 성능 트레이드 오프는 있을지라도
    // 구현상의 이점을 얻을 수 있을 것이다.
}
