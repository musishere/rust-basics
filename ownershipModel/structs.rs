struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("mustafabukhari333@gmail.com"),
        username: String::from("Musishere"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.name = String::from("Mufi");

    let user2 = build_user(String::from("hello@outlook.com"), String::from("helloHere"));

    let user = User {
        email: String::from("hello@outlook.com"),
        username: String::from("helloHere"),
        ..user2,
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
}
