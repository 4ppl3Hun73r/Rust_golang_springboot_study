fn main() {
    println!("Hello, world!");
    another_function(5, 10);

    // compile error let x = (let y = 6);

    let x = 5;
    let y = {
        let x = 3;
        //x + 1; 뒤에 ; 을 넣으면 반환이 아니게 됨
        x + 1
    };

    println!("The value of x is :{}", x);
    println!("The value of y is :{}", y);

    println!("The five {}", five());
    println!("The plus_one {}", plus_one(five()));
}

fn five() -> i32 {
    5 // or return 5;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
