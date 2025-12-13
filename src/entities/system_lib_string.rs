use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct SystemLibString<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for SystemLibString<'a> {
    fn r(&'a self) -> Option<String> {
        Some(self.0.content.replace("<", "").replace(">", ""))
    }
}
