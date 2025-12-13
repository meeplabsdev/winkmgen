use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct CloseContext<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for CloseContext<'a> {
    fn r(&'a self) -> Option<String> {
        None
    }
}
