#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}

enum Point3<T> {
    Ok(T),
    Err(T),
}

fn main() {
    let integer = Point {x:5, y: 10};
    let float = Point{x:1.0, y:4.0};

    println!("{:?}", integer);
    println!("{:?}", integer.x());
    println!("{:?}", float);

    // let compile_error = Point{x: 5, y: 1.4};
    let works = Point2{x: 5, y: 1.4};
    println!("{:?}", works);


    let p1 = Point2 {x: 5, y: 10.4};
    let p2 = Point2 {x: "Hello", y: 'c'};

    println!("{:?}, {:?}", p1, p2);
    let p3 = p1.mixup(p2);
    println!("{:?}", p3);

}
