use crate::entities::{Entity, Entityable};

#[allow(unused)]
pub struct PreprocDef<'a> {
    entity: &'a Entity<'a>,
}

impl<'a> Entityable<'a> for PreprocDef<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!(
            "define!({});",
            self.entity.descend_until(1 /* identifier */, 1)?.r()?
        ))
    }
}
