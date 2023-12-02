#[derive(Debug)]
struct UserProfile {
    username: String,
    age: u32,
    email: String,
}

impl UserProfile {
    fn new(username: String, age: u32, email: String) -> Self {
        Self {
            username,
            age,
            email,
        }
    }

    fn update_profile(&mut self, fields: &Vec<UserProfileFields>) {
        for field in fields {
            match field {
                UserProfileFields::UserName(x) => self.username = x.to_string(),
                UserProfileFields::Age(x) => self.age = *x,
                UserProfileFields::Email(x) => self.email = x.to_string(),
            }
        }
    }
}

#[derive(Debug)]
enum UserProfileFields {
    UserName(String),
    Age(u32),
    Email(String),
}

fn main() {
    let mut user1 = UserProfile::new(String::from("Terry"), 29, String::from("terbeauj@mail.com"));
    dbg!(&user1);

    let fields_vector: Vec<UserProfileFields> = vec![
        UserProfileFields::UserName(String::from("Socrate")),
        UserProfileFields::Age(3),
        UserProfileFields::Email(String::from("socratelechat@mail.com")),
    ];

    user1.update_profile(&fields_vector);
    dbg!(&user1);
    println!("{:?}", fields_vector);
}

// TODO: A lot of concepts in this one, will need to come back and review it !
