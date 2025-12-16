use crate::{
    entities::Entityable,
    entity::{pEntity, vEntity},
};

#[allow(unused)]
pub struct FunctionDeclarator<'a> {
    entity: pEntity<'a>,
    children: vEntity<'a>,

    name: Option<String>,
    struct_name: Option<String>,
}

impl<'a> Entityable<'a> for FunctionDeclarator<'a> {
    fn new(entity: pEntity<'a>) -> Self {
        Self {
            entity,
            children: entity.children(),

            name: entity.child(0).and_then(|e| e.r()),
            struct_name: entity
                .depth_first_ascend(&[303 /* struct_specifier */], 0)
                .and_then(|e| e.depth_first_descend(&[542 /* type_identifier */], 1))
                .and_then(|e| e.r()),
        }
    }

    fn r(&'a self) -> Option<String> {
        let name = self.name.clone()?;

        // If the function name is the same as the struct name (constructor)
        if let Some(n) = self.struct_name.clone()
            && n == name
        {
            return Some(String::from("pub fn new() -> Self"));
        }

        // destructor
        if name == "drop" {
            return Some(String::from("fn drop(&mut self)"));
        }

        Some(format!("fn {name}{}", self.entity.child(1)?.r()?))
    }
}
