use crate::entities::{Entity, Entityable};

#[allow(unused)]
pub struct SystemLibString<'a> {
    entity: &'a Entity<'a>,
}

impl<'a> Entityable<'a> for SystemLibString<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        let mut res = self.entity.content.clone();
        res = res.strip_prefix("<").unwrap_or(&res).into();
        res = res.strip_suffix(">").unwrap_or(&res).into();

        Some(res)
    }
}
