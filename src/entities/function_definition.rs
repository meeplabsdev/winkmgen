use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct FunctionDefinition<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,

    declaration: Option<pEntity<'a>>,
    content: Option<pEntity<'a>>,
}

impl<'a> Entityable<'a> for FunctionDefinition<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self {
            entity,
            children: entity.children(),

            declaration: entity.depth_first_descend(&[286 /* function_declarator */], 1),
            content: entity.depth_first_descend(&[295 /* compound_statement */], 1),
        }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!("{} {}", self.declaration?.r()?, self.content?.r()?))
    }
}
