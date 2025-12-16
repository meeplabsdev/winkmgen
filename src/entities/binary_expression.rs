use crate::{
    entities::{Entity, Entityable},
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct BinaryExpression<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,
}

impl<'a> Entityable<'a> for BinaryExpression<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self {
            entity,
            children: entity.children(),
        }
    }

    fn r(&'a self) -> Option<String> {
        Some(
            self.children
                .iter()
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}
