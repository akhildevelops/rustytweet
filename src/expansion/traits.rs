// pub trait Expansion {
//     fn get_fields(&self) -> &[&str];
//     fn get_identifier(&self) -> String;
//     fn get_fields_identifier(&self) -> String;
// }

pub enum Expansion<'a> {
    User(&'a [&'a str]),
    Tweet(&'a [&'a str]),
}

impl<'a> Expansion<'a> {
    pub fn get_identifier(&self) -> String {
        match self {
            &Self::Tweet(_) => "".to_string(),
            &Self::User(_) => "author_id".to_string(),
        }
    }
    pub fn get_fields(&self) -> &'a [&'a str] {
        match self {
            &Self::Tweet(x) => x,
            &Self::User(x) => x,
        }
    }
    pub fn get_fields_identifier(&self) -> String {
        match self {
            &Self::Tweet(_) => "tweet.fields".to_string(),
            &Self::User(_) => "user.fields".to_string(),
        }
    }
}
