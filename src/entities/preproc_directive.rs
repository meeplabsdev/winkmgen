use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct PreprocDirective<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for PreprocDirective<'a> {
    fn r(&'a self) -> Option<String> {
        Some(self.0.content.split_once('#')?.1.into())
    }
}
