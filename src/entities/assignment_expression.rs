use crate::{
    entities::{Entity, ToRust},
    get_from_range,
};

#[allow(unused)]
pub struct AssignmentExpression<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for AssignmentExpression<'a> {
    fn r(&'a self) -> Option<String> {
        let mut locals = Vec::new();
        if let Some(list) = self.0.ascend_until(305) {
            for child in list.children(&mut list.walk()) {
                match child.kind_id() {
                    307 => locals.push(get_from_range(child.child(1)?.byte_range())),
                    _ => {}
                }
            }
        }

        let mut children = Vec::new();
        for child in self.0.children.iter() {
            children.push(match child.node.kind_id() {
                1 => {
                    if locals.contains(&child.content) {
                        format!("self.{}", child.r()?)
                    } else {
                        child.r()?
                    }
                }
                _ => child.r()?,
            });
        }

        Some(format!("{};", children.join(" ")))
    }
}
