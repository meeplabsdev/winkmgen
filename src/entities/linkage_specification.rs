use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct LinkageSpecification<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,
}

impl<'a> Entityable<'a> for LinkageSpecification<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self {
            entity,
            children: entity.children(),
        }
    }

    fn r(&'a self) -> Option<String> {
        self.children.get(2)?.r()
    }
}
