use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct ParameterList<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,
}

impl<'a> Entityable<'a> for ParameterList<'a> {
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
                .half_open_offset(1, 1)
                .iter()
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join(", ")
        ))
    }
}
