use crate::{
    entities::{Entity, ToRust},
    get_from_range,
};

#[allow(unused)]
pub struct FunctionDeclarator<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for FunctionDeclarator<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 2 {
            return None;
        }

        let name = children.get(0)?.r()?;
        if let Some(p) = self.0.ascend_until(303)
            && name == get_from_range(p.child(1)?.byte_range())
        {
            return Some(String::from("pub fn new() -> Self"));
        }

        if name == String::from("drop") {
            return Some(String::from("fn drop(&mut self)"));
        }

        Some(format!("fn {name}{}", children.get(1)?.r()?))
    }
}
