use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct CompoundStatement<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for CompoundStatement<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 2 {
            return None;
        }

        Some(format!(
            "{{\n\t{}\n}}",
            children[1..children.len() - 1]
                .iter()
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join("\n\t"),
        ))
    }
}
