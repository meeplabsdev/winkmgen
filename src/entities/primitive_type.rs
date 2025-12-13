use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct PrimitiveType<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for PrimitiveType<'a> {
    fn r(&'a self) -> Option<String> {
        Some(self.0.content.clone())
    }
}
