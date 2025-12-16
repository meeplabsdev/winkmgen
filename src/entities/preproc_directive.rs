use crate::entities::{Entity, Entityable};

#[allow(unused)]
pub struct PreprocDirective<'a> {
    entity: &'a Entity<'a>,
}

impl<'a> Entityable<'a> for PreprocDirective<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        Some(self.entity.content.split_once('#')?.1.into())
    }
}
