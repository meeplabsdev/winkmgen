use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct FieldIdentifier<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for FieldIdentifier<'a> {
    fn r(&'a self) -> Option<String> {
        Some(self.0.content.clone())
    }
}
