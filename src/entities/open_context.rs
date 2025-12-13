use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct OpenContext<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for OpenContext<'a> {
    fn r(&'a self) -> Option<String> {
        None
    }
}
