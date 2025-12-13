use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct TypeDefinition<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for TypeDefinition<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 5 {
            return None;
        }

        Some(format!(
            "{}\ntype {} = {name};\ntype {} = *mut {name};",
            children.get(1)?.r()?,
            children.get(2)?.r()?,
            children.get(4)?.r()?,
            name = children.get(1)?.children.get(1)?.r()?,
        ))
    }
}
