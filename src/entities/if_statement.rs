use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct IfStatement<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for IfStatement<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 3 {
            return None;
        }

        Some(format!(
            "if {} {}",
            children.get(1)?.r()?,
            children
                .get(2)?
                .r()?
                .lines()
                .collect::<Vec<&str>>()
                .join("\n\t")
        ))
    }
}
