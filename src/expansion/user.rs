// use super::traits::Expansion;
// pub struct User<'a> {
//     fields: &'a [&'a str],
// }

// impl<'a> User<'a> {
//     pub fn new(fields: &'a [&'a str]) -> Self {
//         Self { fields }
//     }
// }

// impl<'a> Expansion for User<'a> {
//     fn get_fields(&self) -> &[&str] {
//         self.fields
//     }
//     fn get_identifier(&self) -> String {
//         "author_id".to_string()
//     }
//     fn get_fields_identifier(&self) -> String {
//         "user.fields".to_string()
//     }
// }
