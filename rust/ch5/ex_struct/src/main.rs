struct Rectangle {
    width: u32,
    height: u32
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let length = 50;
    let width = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length, width)
    );

    // refactoring tuples
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect)
    );


    // Refatoring structs
    let rect = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect)
    );

    // Rectangle` doesn't implement `std::fmt::Display
    // println!("The area of the rectangle is {} square pixels.", rect);
    // `Rectangle` doesn't implement `std::fmt::Debug`
    // println!("The area of the rectangle is {:?} square pixels.", rect);

    let rect2 = Rect {width: 30, height: 50};
    println!("rect is {:?}", rect2);
    println!("rect is {:#?}", rect2); // 조덤 읽기 쉽게 (엔터 쳐줌)

}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
