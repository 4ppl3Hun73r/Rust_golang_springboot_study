struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

/* missing lifetime specifier 에러 발생
struct User2 {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool
}
*/

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
        // not_exist_value : 1 // compile error
    };

    // donesn't allow user1.email = String::from("anotheremail@example.com");

    // compile error let user2 = build_user("someone2@example.com", "someusername234");
    let user2 = build_user(String::from("someone2@example.com"), String::from("someusername234"));
    let user3 = build_user2(String::from("someone3@example.com"), String::from("someusername345"));

    let user4 = User {
        email: String::from("someone4@example.com"),
        username: String::from("someusername456"),
        ..user3 // 남은 부분은 user3것으로 채워짐
    };


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // 동일하지만 다름

/*
    let user5 = User {
        email: "somone@example.com", // missing lifetime specifire error발생 
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
    */
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}

fn build_user2(email: String, username: String) -> User {
    User {
        email,  // field init shorthand 동일 이름일 경우
        username,
        active: true,
        sign_in_count: 1
    }
}
