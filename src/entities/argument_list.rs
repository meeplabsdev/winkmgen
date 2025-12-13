use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct ArgumentList<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for ArgumentList<'a> {
    fn r(&'a self) -> Option<String> {
        Some(format!(
            "({})",
            self.0
                .children
                .iter()
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join(", "),
        ))
    }
}
