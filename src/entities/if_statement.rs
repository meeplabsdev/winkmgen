use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct IfStatement<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for IfStatement<'a> {
    fn r(&'a self) -> Option<String> {
        Some(String::from("IF"))
    }
}
