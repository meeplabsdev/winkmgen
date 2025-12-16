use crate::{
    entities::{Entity, Entityable},
    entity::pEntity,
};

#[allow(unused)]
pub struct IfStatement<'a> {
    entity: pEntity<'a>,

    condition: Option<pEntity<'a>>,
    content: Option<pEntity<'a>>,
}

impl<'a> Entityable<'a> for IfStatement<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self {
            entity,
            condition: entity.depth_first_descend(&[441 /* condition_clause */], 1),
            content: entity.depth_first_descend(&[295 /* compound_statement */], 1),
        }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!(
            "if {} {}",
            self.condition?.r()?,
            self.content?
                .r()?
                .lines()
                .collect::<Vec<&str>>()
                .join("\n\t")
        ))
    }
}
