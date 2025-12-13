use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct BinaryExpression<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for BinaryExpression<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 3 {
            return None;
        }

        Some(
            children
                .iter()
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}
