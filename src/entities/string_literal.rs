use crate::entities::{Entity, Entityable};

#[allow(unused)]
pub struct StringLiteral<'a> {
    entity: &'a Entity<'a>,
}

impl<'a> Entityable<'a> for StringLiteral<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!(
            "\"{}\"",
            self.entity
                .descend_until(170 /* string_content */, 1)?
                .r()?
        ))
    }
}
