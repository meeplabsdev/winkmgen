use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct InitDeclarator<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,

    declaration: Option<pEntity<'a>>,
}

impl<'a> Entityable<'a> for InitDeclarator<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self {
            entity,
            children: entity.children(),

            declaration: entity.depth_first_ascend(&[255 /* declaration */], 0),
        }
    }

    fn r(&'a self) -> Option<String> {
        let type_identifier = self
            .declaration?
            .depth_first_descend(&[542 /* type_identifier */], 1)?
            .r()?;

        let mut res = Vec::new();
        for child in self.children.iter() {
            res.push(match child.kind() {
                1 => format!("{}: {}", child.r()?, type_identifier),
                _ => child.r()?,
            });
        }

        Some(res.join(" "))
    }
}
