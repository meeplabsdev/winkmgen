use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct ParenthesizedExpression<'a> {
    entity: pEntity<'a>,
}

impl<'a> Entityable<'a> for ParenthesizedExpression<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!(
            "({})",
            self.entity
                .depth_first_descend(&[341 /* binary_expression */], 1)?
                .r()?
        ))
    }
}
