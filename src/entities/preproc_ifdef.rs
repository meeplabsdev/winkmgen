use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct PreprocIfdef<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for PreprocIfdef<'a> {
    fn r(&'a self) -> Option<String> {
        let children = &self.0.children;
        if children.len() < 3 {
            return None;
        }

        Some(format!(
            "preproc_if!(r#{}, {}, {{\n\t{}\n}});",
            match children.get(0)?.node.kind_id() {
                13 => "ifndef",
                _ => return None,
            },
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
