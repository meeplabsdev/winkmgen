use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct AssignmentExpression<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,
}

impl<'a> Entityable<'a> for AssignmentExpression<'a> {
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
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}
