use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct FieldDeclaration<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for FieldDeclaration<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 2 {
            return None;
        }

        if children.get(1)?.node.kind_id() == 286 {
            return Some(format!(
                "\t{} -> {},",
                children.get(1)?.r()?,
                children.get(0)?.r()?
            ));
        }

        Some(format!(
            "\t{}: {},",
            children.get(1)?.r()?,
            children.get(0)?.r()?
        ))
    }
}
