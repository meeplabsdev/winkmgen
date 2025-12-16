use crate::entities::{Entity, EntityList, Entityable};

#[allow(unused)]
pub struct TypeQualifier<'a> {
    entity: &'a Entity<'a>,
    children: &'a EntityList<'a>,
}

impl<'a> Entityable<'a> for TypeQualifier<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self {
            entity,
            children: &entity.children,
        }
    }

    fn r(&'a self) -> Option<String> {
        self.children.index(0)?.r()
    }
}
