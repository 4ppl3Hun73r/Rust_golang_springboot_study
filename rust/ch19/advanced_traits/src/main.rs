use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// trait Add<RHS=Self> { RHS -> right hand side
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });

    let mill = Millimeters(30);
    let meter = Meters(10);
    println!("{:?}", mill + meter);

    let person = Human;
    Pilot::fly(&person);    // 명확하게 호출
    Wizard::fly(&person);
    person.fly();   // 명확하지 않아

    // dog 호출
    println!("A baby dog is called a {}", Dog::baby_name());

    // 컴파일 에러 self가 없어서 animal만으로는 안됨
    //println!("A baby dog is called a {}", Animal::baby_name());

    // 정확히 명시해줌
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}


/*
pub trait Iterator {
    // type 은 트레잇을 구현하는 시점에 구체적으로 지정됨
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
*/
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// 제네릭과의 차이점은 매번 type을 명시해줄 필요 가 없음