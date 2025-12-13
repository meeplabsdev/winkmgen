use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct CallExpression<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for CallExpression<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 2 {
            return None;
        }

        Some(
            children
                .iter()
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
