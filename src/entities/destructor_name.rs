use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct DestructorName {}

impl<'a> Entityable<'a> for DestructorName {
    fn new(_entity: pEntity<'a>) -> Self {
        Self {}
    }

    fn r(&'a self) -> Option<String> {
        Some(String::from("drop"))
    }
}
