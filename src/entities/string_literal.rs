use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct StringLiteral<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for StringLiteral<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 2 {
            return None;
        }

        children.get(1)?.r()
    }
}
