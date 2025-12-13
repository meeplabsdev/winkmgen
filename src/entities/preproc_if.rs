use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct PreprocIf<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for PreprocIf<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 3 {
            return None;
        }

        Some(format!(
            "preproc_if!(r#if, {} {{\n\t{}\n}});",
            children.get(1)?.r()?,
            children[2..children.len() - 1]
                .iter()
                .filter_map(|c| c.r())
                .map(|r| r.lines().collect::<Vec<&str>>().join("\n\t"))
                .collect::<Vec<String>>()
                .join("\n\t")
        ))
    }
}
