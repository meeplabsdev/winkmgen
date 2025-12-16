use crate::entities::{Entity, EntityList, Entityable};

#[allow(unused)]
pub struct TypeDefinition<'a> {
    entity: &'a Entity<'a>,
    children: &'a EntityList<'a>,
}

impl<'a> Entityable<'a> for TypeDefinition<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self {
            entity,
            children: &entity.children,
        }
    }

    fn r(&'a self) -> Option<String> {
        if !self.children.has_capacity(3) {
            return None;
        }

        let name;
        let n = self.children.index(1)?;
        match n.kind {
            542 => name = n.r()?,
            303 => name = n.descend_until(542 /* type_identifier */, 1)?.r()?,
            _ => return None,
        }

        let mut result = Vec::new();
        for child in self.children.inclusive_offset(2, 1) {
            if child.r().is_none() {
                continue;
            }

            let c = child.r()?;
            result.push(match child.kind {
                282 => format!("type {c} = *mut {name};"),
                _ => format!("type {c} = {name};"),
            });
        }

        Some(format!("{name}\n{}", result.join("\n")))
    }
}
