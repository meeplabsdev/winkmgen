use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct ParameterDeclaration<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,

    type_identifier: Option<pEntity<'a>>,
    name: Option<pEntity<'a>>,
}

impl<'a> Entityable<'a> for ParameterDeclaration<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self {
            entity,
            children: entity.children(),

            type_identifier: entity.depth_first_descend(&[542 /* type_identifier */], 1),
            name: entity
                .depth_first_descend(&[1 /* identifier */, 485 /* qualified_identifier */], 1),
        }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!(
            "{}: {}",
            self.type_identifier?.r()?,
            self.name?.r()?
        ))
    }
}
