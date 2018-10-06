extern crate communicator;

fn main() {
    // client connect 가 비공개 이면 실행 불가
    // communicator::client::connect();
    communicator::client::connect();
}
