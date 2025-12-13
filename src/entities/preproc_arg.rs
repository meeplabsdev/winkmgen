use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct PreprocArg<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for PreprocArg<'a> {
    fn r(&'a self) -> Option<String> {
        Some(self.0.content.trim().into())
    }
}
