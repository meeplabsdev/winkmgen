use crate::entities::{Entity, Entityable};

#[allow(unused)]
pub struct StringContent<'a> {
    entity: &'a Entity<'a>,
}

impl<'a> Entityable<'a> for StringContent<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        Some(self.entity.content.clone())
    }
}
