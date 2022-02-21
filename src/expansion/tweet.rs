// use super::traits::Expansion;
// pub struct Tweet<'a> {
//     fields: &'a [&'a str],
// }

// impl<'a> Tweet<'a> {
//     pub fn new(fields: &'a [&'a str]) -> Self {
//         Self { fields }
//     }
// }

// impl<'a> Expansion for Tweet<'a> {
//     fn get_fields(&self) -> &[&str] {
//         self.fields
//     }

//     fn get_identifier(&self) -> String {
//         "".to_string()
//     }
//     fn get_fields_identifier(&self) -> String {
//         "tweet.fields".to_string()
//     }
// }
