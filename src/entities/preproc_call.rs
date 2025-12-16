use crate::{entities::Entityable, entity::pEntity};

#[allow(unused)]
pub struct PreprocCall<'a> {
    entity: pEntity<'a>,

    directive: Option<pEntity<'a>>,
    arg: Option<pEntity<'a>>,
}

impl<'a> Entityable<'a> for PreprocCall<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self {
            entity,
            directive: entity.depth_first_descend(&[19 /* preproc_directive */], 1),
            arg: entity.depth_first_descend(&[18 /* preproc_arg */], 1),
        }
    }

    fn r(&'a self) -> Option<String> {
        Some(format!("{}!({});", self.directive?.r()?, self.arg?.r()?))
    }
}
