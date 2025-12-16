use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct FieldIdentifier<'a> {
    entity: pEntity<'a>,
}

impl<'a> Entityable<'a> for FieldIdentifier<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        Some(self.entity.content())
    }
}
