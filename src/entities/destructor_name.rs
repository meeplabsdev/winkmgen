use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct DestructorName<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for DestructorName<'a> {
    fn r(&'a self) -> Option<String> {
        // let children = &self.0.children;
        // if children.len() < 2 {
        //     return None;
        // }

        // Some(format!("drop_{}", children.get(1)?.r()?))
        Some(String::from("drop"))
    }
}
