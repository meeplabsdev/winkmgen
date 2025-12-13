use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct Comment<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for Comment<'a> {
    fn r(&'a self) -> Option<String> {
        Some(self.0.content.clone())
        // None
    }
}
