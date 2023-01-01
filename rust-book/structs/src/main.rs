fn main() {
    let mut user1 = User {
        email: String::from("email@example.com"),
        username: String::from("username1"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1 email is {}", user1.email);

    user1.username = String::from("newUsername");

    println!("user1 username is {}", user1.username);

    let user2 = build_user("email2@example.com".to_string(), "username2".to_string());
    println!("user2 email is {}", user2.email);
    println!("user2 username is {}", user2.username);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
