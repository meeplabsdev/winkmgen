use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct CallExpression<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for CallExpression<'a> {
    fn r(&'a self) -> Option<String> {
        Some(String::from("CALL"))
    }
}
