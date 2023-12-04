use crate::user_profile_fields::UserProfileFields;

#[derive(Debug)]
pub struct UserProfile {
    pub username: String,
    pub age: u32,
    pub email: String,
}

impl UserProfile {
    pub fn new(username: String, age: u32, email: String) -> Self {
        Self {
            username,
            age,
            email,
        }
    }

    pub fn update_profile(&mut self, fields: &Vec<UserProfileFields>) {
        for field in fields {
            match field {
                UserProfileFields::UserName(x) => self.username = x.to_string(),
                UserProfileFields::Age(x) => self.age = *x,
                UserProfileFields::Email(x) => self.email = x.to_string(),
            }
        }
    }
}
