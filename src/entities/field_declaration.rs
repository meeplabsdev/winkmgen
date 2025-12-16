use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct FieldDeclaration<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,

    type_identifier: Option<pEntity<'a>>,
    field_contents: Option<pEntity<'a>>,
}

impl<'a> Entityable<'a> for FieldDeclaration<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self {
            entity,
            children: entity.children(),

            type_identifier: entity.child(0),
            field_contents: entity.depth_first_descend(
                &[
                    538, /* field_identifier */
                    286, /* function_declarator */
                ],
                1,
            ),
        }
    }

    fn r(&'a self) -> Option<String> {
        Some(match self.field_contents?.kind() {
            286 => format!(
                "\t{} -> {},",
                self.field_contents?.r()?,
                self.type_identifier?.r()?
            ),
            _ => format!(
                "\t{}: {},",
                self.field_contents?.r()?,
                self.type_identifier?.r()?
            ),
        })
    }
}
