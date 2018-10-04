enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
    fn_p: fn(IpAddrKind)  // function pointer 있나해서 찾아봤더니 역시 있군 :)
}

enum IpAddre {
    V4(String), // V4(u8, u8, u8, u8), 처럼 값이 달라도 상관 없음
    V6(String),
}

struct Ipv4Addr {
    // details elided
}

struct Ipv6Addr {
    // details elided
}

enum IpvxAddr {
    // enum
}

// variant 로 어떠한 데이터도 넣을수 있음
enum IpAddr3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    VX(IpvxAddr)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 익명 구조체
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // 메서드 정의
    }
}

struct QuireMessage; //유닛 구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체



fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
        fn_p : route
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
        fn_p : route
    };


    let home = IpAddre::V4(String::from("127.0.0.1"));
    let loopback = IpAddre::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // enum도 method가 정의되고, strunt가 되고... 어떤 차이가 있을까?
}

fn route(ip_type: IpAddrKind) {
    
}
