use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct Declaration<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,
}

impl<'a> Entityable<'a> for Declaration<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self {
            entity,
            children: entity.children(),
        }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!(
            "{};",
            self.children
                .iter()
                .filter(|c| c.kind() != 542 /* type_identifier */)
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join(" ")
        ))
    }
}
