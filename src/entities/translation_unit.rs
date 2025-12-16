use crate::entities::{Entity, EntityList, Entityable};

#[allow(unused)]
pub struct TranslationUnit<'a> {
    entity: &'a Entity<'a>,
    children: &'a EntityList<'a>,
}

impl<'a> Entityable<'a> for TranslationUnit<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self {
            entity,
            children: &entity.children,
        }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!(
            "#![allow(nonstandard_style, unused)]\n\n{}\n",
            self.children
                .iter()
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join("\n")
        ))
    }
}
