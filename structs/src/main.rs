
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs 
struct Color(i32,i32,i32);

struct Point(i32,i32,i32);

fn build_user(email: String, username: String) -> User {
    // Eine neue instanz erstellen 
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// unit structs 
struct AlwaysEqual;


fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("bob123"),
        email: String::from("bob@example.com"),
        sign_in_count: 1,
    };

    build_user("bob@example.com".to_string(),"bob123".to_string());

    user1.email = String::from("differentbob@example.com");

    println!("{}",user1.email);

    
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@exaple.com"),
        sign_in_count: user1.sign_in_count
    };

    let user3 = User {
        ..user2
    };

    // Tuple structs 

    let black = Color(0,0,0);
    let origin = Point(0,5,0);

    println!("{}",origin.0);
    println!("{}",origin.1);

    // unit structs 
    let subject = AlwaysEqual;

}
