use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct ExpressionStatement<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for ExpressionStatement<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 1 {
            return None;
        }

        children.get(0)?.r()
    }
}
