use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct PointerDeclaration<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for PointerDeclaration<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 2 {
            return None;
        }

        Some(children.get(1)?.r()?)
    }
}
