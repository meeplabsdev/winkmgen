use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct StructSpecifier<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for StructSpecifier<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 3 {
            return None;
        }

        let struct_name = children.get(1)?.r()?;
        let mut functions = Vec::new();
        for f in children
            .get(2)?
            .children
            .iter()
            .filter(|c| c.node.kind_id() == 254)
        {
            if f.children.get(0)?.children.get(0)?.r()? == String::from("drop") {
                functions.push(format!(
                    "impl Drop for {struct_name} {{\n\t{}\n}}",
                    f.r()?.lines().collect::<Vec<&str>>().join("\n\t")
                ))
            } else {
                functions.push(format!(
                    "impl {struct_name} {{\n\t{}\n}}",
                    f.r()?.lines().collect::<Vec<&str>>().join("\n\t")
                ))
            }
        }

        if functions.len() > 0 {
            functions.insert(0, String::new());
        }

        Some(format!(
            "#[repr(C)]\npub struct {struct_name} {{\n{}\n}}{}",
            children.get(2)?.r()?,
            functions.join("\n\n")
        ))
    }
}
