//mod client2;
//

// private 은 같은 파일 내의 부모 모듈 / 자식 모듈에서 접근
// pub 이면 어디에서든 접근 가능

pub mod client;
pub mod network;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        // errorclient::connect();
        ::client::connect();
        super::client::connect();
    }

    use super::client;

    #[test]
    fn it_works2() {
        client::connect();
    }
}
/*
mod  network {
    fn connect() {
        
    }
    mod client {
        fn connect() { }
    }
    mod server {
        fn connect() {}
    }
}

mod client {
    fn connect() {

    }
}
*/
