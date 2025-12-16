use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct GreaterThan {}

impl<'a> Entityable<'a> for GreaterThan {
    fn new(_entity: pEntity<'a>) -> Self {
        Self {}
    }

    fn r(&'a self) -> Option<String> {
        Some(String::from(">"))
    }
}
