struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // drop 트레잇 구현으로 drop 시점에 실행됨
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer{data: String::from("my stuff")};
    let d = CustomSmartPointer{data: String::from("other stuff")};

    println!("CustomSmartPointer created.");
    // explicit destructor calls not allowed
    // 명시적 호출은 일단은 허용 되지 않음 c.drop();
    drop(c); // std::mem::drop 은 사용 가능
    println!("CustomSmartPointer dropped before the end of main.");
}
