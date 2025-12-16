use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct Const {}

impl<'a> Entityable<'a> for Const {
    fn new(_entity: pEntity<'a>) -> Self {
        Self {}
    }

    fn r(&'a self) -> Option<String> {
        Some(String::from("const"))
    }
}
