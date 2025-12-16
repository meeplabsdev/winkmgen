use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct NumberLiteral<'a> {
    entity: pEntity<'a>,
}

impl<'a> Entityable<'a> for NumberLiteral<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        Some(self.entity.content())
    }
}
