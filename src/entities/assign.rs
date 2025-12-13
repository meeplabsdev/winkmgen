use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct Assign<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for Assign<'a> {
    fn r(&'a self) -> Option<String> {
        Some(String::from("="))
    }
}
