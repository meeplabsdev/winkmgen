use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct StringContent<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for StringContent<'a> {
    fn r(&'a self) -> Option<String> {
        Some(self.0.content.clone())
    }
}
