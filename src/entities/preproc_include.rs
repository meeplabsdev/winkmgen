use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct PreprocInclude<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for PreprocInclude<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 2 {
            return None;
        }

        Some(format!(
            "mod {};",
            children.get(1)?.r()?.split_terminator('.').next()?
        ))
    }
}
