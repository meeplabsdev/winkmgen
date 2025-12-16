use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct Comment<'a> {
    entity: pEntity<'a>,
}

impl<'a> Entityable<'a> for Comment<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        Some(self.entity.content())
    }
}
