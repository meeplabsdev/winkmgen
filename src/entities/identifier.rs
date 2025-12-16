use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct Identifier<'a> {
    entity: pEntity<'a>,
}

impl<'a> Entityable<'a> for Identifier<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        Some(self.entity.content())
    }
}
