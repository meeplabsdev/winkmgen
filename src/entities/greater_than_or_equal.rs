use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct GreaterThanOrEqual<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for GreaterThanOrEqual<'a> {
    fn r(&'a self) -> Option<String> {
        Some(String::from(">="))
    }
}
