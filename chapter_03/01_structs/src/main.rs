#![allow(unused)]

struct User {
    active: bool,
    sign_in_count: u32,
    username: String,
    email: String,
    phone: u32,
}

fn main() {
    let user = User {
        active: true,
        sign_in_count: 1,
        username: "gemz".to_string(),
        email: "gemz@gmail.com".to_string(),
        phone: 911,
    };

    let mut user2 = User {
        active: true,
        sign_in_count: 1,
        username: "gemz".to_string(),
        email: "gemz@gmail.com".to_string(),
        phone: 911,
    };

    user2.email = "someone_else@example.com".to_string();

    let user3 = build_user(user2.username, user2.email, 711);

    let user4 = User {
        email: String::from("bot@bot.com"),
        ..user3
    };
}

fn build_user(username: String, email: String, phone: u32) -> User {
    User {
        active: true,
        sign_in_count: 1,
        username,
        email,
        phone,
    }
}
