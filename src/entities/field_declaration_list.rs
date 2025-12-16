use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct FieldDeclarationList<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,
}

impl<'a> Entityable<'a> for FieldDeclarationList<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self {
            entity,
            children: entity.children(),
        }
    }

    fn r(&'a self) -> Option<String> {
        Some(
            self.children
                .iter()
                .filter(|c| c.kind() == 307 /* field_declaration */)
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
