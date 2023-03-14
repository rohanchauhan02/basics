struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

// Method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

//Associated function

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

pub fn structs_tuple() {
    struct_in_rust();
    let rect = (4, 5);

    println!("{}", area_tuple(rect));
    let rect2 = Rectangle {
        length: 5,
        width: 6,
    };
    println!("{:#?}", rect2);
    println!("{}", area_struct(&rect2));

    println!("Area = {}", rect2.area());
    println!(
        "Is the rec2 contain given rec = {}",
        rect2.can_hold(&Rectangle {
            length: 2,
            width: 3
        })
    );

    let rec3 = Rectangle::square(3);
    println!("Area of rec3 = {}", rec3.area());
}

fn struct_in_rust() {
    let mut user1 = User {
        username: String::from("Rohan"),
        email: String::from("singhrohankumar7@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.username = String::from("Mohan");

    println!(
        "{},{},{},{}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
    let user2 = build_user(String::from("test@gmail.com"), String::from("test"));
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area_tuple(dimention: (u32, u32)) -> u32 {
    dimention.0 * dimention.1
}
fn area_struct(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}
