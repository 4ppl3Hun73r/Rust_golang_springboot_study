pub trait Draw {
    fn draw(&self);
    // self를 반환하는 메소드는 다형성 용도로 사용할 수 없다.
    fn test(&self, time: u32) -> Self;
    // 같은 예로 제네릭 타입의 메게변수 또한 포함 할 수 없다.
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
            component.test(10);
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{}", self.width);
    }
    fn test(&self, time: u32) -> Self {
        println!("{}", time);
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
