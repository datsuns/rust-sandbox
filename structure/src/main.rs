struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}

// タプル構造体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn generate_instance_of_structure() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("somveusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_by_shorten_grammer(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_from_existed_instance() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("somveusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2_normal_grammer = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user2_shorten_grammer = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        ..user1
    };
}

fn main() {
    generate_instance_of_structure();
}
