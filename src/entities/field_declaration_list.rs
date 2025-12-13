use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct FieldDeclarationList<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for FieldDeclarationList<'a> {
    fn r(&'a self) -> Option<String> {
        Some(
            self.0
                .children
                .iter()
                .filter(|c| c.node.kind_id() == 307) // field_declaration
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
