use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct PointerExpression<'a> {
    entity: pEntity<'a>,
}

impl<'a> Entityable<'a> for PointerExpression<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!(
            "&{}",
            self.entity
                .depth_first_descend(&[1 /* identifier */], 1)?
                .r()?
        ))
    }
}
