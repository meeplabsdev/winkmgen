use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct PointerExpression<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for PointerExpression<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 2 {
            return None;
        }

        Some(format!("&{}", children.get(1)?.r()?))
    }
}
