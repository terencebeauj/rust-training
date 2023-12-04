mod user_profile;
mod user_profile_fields;

use crate::user_profile::UserProfile;
use crate::user_profile_fields::UserProfileFields;

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
