#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

// 함수 정의를 위해서 implementation 을 해야함
impl Rectangle {
    // self 역시 소유권이전, 빌려오기, 변경가능 빌려오기 가능
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

// impl을 2번해도 둘다 반영됨... 
// 해당 부분의 유용함은 10장에서 좀더 다룸
impl Rectangle {
    // Associated Function
    // String:;from("str"); 
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };

    // 두개는 동일하게 동작 automatic referencing and dereferencing
    println!("{}, {}", p1.distance(&p2), (&p1).distance(&p2));


    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect.can_hold(&rect3));

    println!("{:?}", Rectangle::square(3));
    // paramter가 없다고 에러남. self 가 없어서 println!("{}", Rectangle::area());
}


// -> 연사자 사용하지 않음 자동 참조 및 역참조 (automatic referencing and dereferencing) 기능을
// 가지고 있음
// object.something()을 호출하면 자동적으로 &나 &mut, *을 붙여서 object가 해당 메소드의 시그니처와
// 맞게 해줌
#[derive(Debug,Copy,Clone)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);

        f64::sqrt(x_squared + y_squared)
    }
}
