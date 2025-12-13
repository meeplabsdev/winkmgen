use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct ParameterDeclaration<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for ParameterDeclaration<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() == 1 {
            return Some(String::new());
        } else if children.len() < 2 {
            return None;
        }

        Some(format!(
            "{}: {}",
            children.get(1)?.r()?,
            children.get(0)?.r()?
        ))
    }
}
