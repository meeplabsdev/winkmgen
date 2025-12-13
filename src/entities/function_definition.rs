use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct FunctionDefinition<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for FunctionDefinition<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 2 {
            return None;
        }

        Some(format!(
            "{} {}",
            children.get(0)?.r()?,
            children
                .get(1)?
                .r()?
                .lines()
                .collect::<Vec<&str>>()
                .join("\n")
        ))
    }
}
