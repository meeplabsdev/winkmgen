use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct ConditionClause<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,
}

impl<'a> Entityable<'a> for ConditionClause<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self {
            entity,
            children: entity.children(),
        }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!(
            "({})",
            self.children
                .iter()
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join(" ")
        ))
    }
}
