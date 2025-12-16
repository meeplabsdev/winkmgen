use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct Assign {}

impl<'a> Entityable<'a> for Assign {
    fn new(_entity: pEntity<'a>) -> Self {
        Self {}
    }

    fn r(&'a self) -> Option<String> {
        Some(String::from("="))
    }
}
