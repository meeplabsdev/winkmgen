use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct TypeIdentifier<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for TypeIdentifier<'a> {
    fn r(&'a self) -> Option<String> {
        Some(self.0.content.clone())
    }
}
