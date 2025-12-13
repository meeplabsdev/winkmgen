use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct ParameterList<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for ParameterList<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 3 {
            return None;
        }

        Some(format!(
            "({})",
            children[1..children.len() - 1]
                .iter()
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join(", ")
        ))
    }
}
