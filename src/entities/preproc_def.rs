use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct PreprocDef<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for PreprocDef<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 2 {
            return None;
        }

        // Some(format!(
        //     "pub static {}: IFDEF = Some(true);",
        //     children.get(1)?.r()?
        // ))

        Some(format!("define!({});", children.get(1)?.r()?))
    }
}
