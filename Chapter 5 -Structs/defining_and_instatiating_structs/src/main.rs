fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let mut userFromMethod = build_user(String::from("example@email.com"), String::from("usernamexample123"));

    userFromMethod.email = String::from("anotheremail@email.com");

    //creates an instance in user2 that has a different value for email but has the same values for the username, active, and sign_in_count fields from user1
    //The "..user1" must come last to specify that any remaining fields should get their values from the corresponding fields in user1
    let mut user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    user3.email = String::from("outroemail@example.com");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}