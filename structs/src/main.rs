struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0
    }
}

fn main() {
    let mut user = build_user(String::from("test-user"), String::from("test@dev.local"));

    user.email = String::from("new-email@dev.local");

    let user2 = User {
        email: String::from("test2@dev.local"),
        username: String::from("test2-user"),
        ..user
    };

    println!("{}, {}, {}, {}", user.email, user.username, user.sign_in_count, user.active);

    println!("{}, {}, {}, {}", user2.email, user2.username, user2.sign_in_count, user2.active);
}
