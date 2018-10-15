use std::net::IpAddr;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

}

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("{:?}", home);

    let _guess = Guess::new(100) ;
    // panic 발생 let _guess = Guess::new(101) ;

    // new가 아니라 바로 생성하면 어차피 검증 못하지 않나?
    // 바로 생성을 막을 방법은 없나? java의 private 생성자 지정처럼...
    let _guess = Guess {
        value: 32
    };
}
