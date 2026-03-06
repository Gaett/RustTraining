use std::fmt;

struct Book {
    title: String,
    author: String,
    pages: u32, //I don't expect a book to be more than u32::MAX
    available: bool,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl fmt::Display for User {
    /// Formats the 'User' struct for display.
    ///
    /// # Arguments
    /// * 'f' - The formatter to write to.
    ///
    /// # Returns
    /// A 'fmt::Result' indicating success or failure.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            //write formatted data into a buffer
            f,
            "User {{ active: {}, username: {}, email: {}, sign_in_count: {} }}",
            self.active,
            self.username,
            self.email,
            self.sign_in_count
        )
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("User")
            .field("active", &self.active)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("sign_in_count", &self.sign_in_count)
            .finish()
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 0,
    }
}

fn test_struct() {
    // tuple as a structure
    let _rect = (200, 500);

    // Plain comment : if we want to modify/update the user later
    let mut user1 = User {
        active: true,
        email: String::from("a@a.a"),
        sign_in_count: 1,
        username: String::from("a"),
    };

    user1.email = String::from("a1@a.a");
    println!("{}", user1);

    let user2 = User {
        email: String::from("b@b.b"),
        ..user1
    }; // This case borrows some data from user1

    let user3 = build_user(String::from("c@c.c"), String::from("c"));

    println!("{:?}", user2);
    println!("{}", user3);

    // Tuple structs
    struct Point(i32, i32, i32);
    let _here = Point(1, 1, 1);

    // Unit-like struct
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

enum IpAddrKind {
    IpV4,
    IpV6,
}

enum IpAddr {
    V4(u8, u8, u8, u8), // 4 Bytes
    V6(String),         // IPv6 as a String
}

struct IpAddrPlus {
    kind: IpAddrKind,
    adress: String,
}

fn route(_ip_kind: IpAddrKind) {
    match _ip_kind {
        IpAddrKind::IpV4 => println!("IPv4"),
        IpAddrKind::IpV6 => println!("IPv6"),
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddr::V4(a, b, c, d) => {
                //We use the declarated order
                write!(f, "{}.{}.{}.{}", a, b, c, d)
            }
            IpAddr::V6(addr) => {
                write!(f, "{}", addr)
            }
        }
    }
}

fn test_enum() {
    let _v4 = IpAddrKind::IpV4;
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
    route(_v4);

    let an_ip = IpAddrPlus {
        kind: IpAddrKind::IpV4,
        adress: String::from("127.0.0.1"),
    };

    // println!("{:?}", _home); Won't work until we've implemented Debug or Display
    println!("{}", _home);
    // println!("{:?}", an_ip); Won't work until we've implemented Debug
}

fn main() {
    test_struct();
    test_enum();
}
