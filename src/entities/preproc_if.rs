use crate::entities::{Entity, EntityList, Entityable};

#[allow(unused)]
pub struct PreprocIf<'a> {
    entity: &'a Entity<'a>,
    children: &'a EntityList<'a>,
}

impl<'a> Entityable<'a> for PreprocIf<'a> {
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

        Some(format!(
            "preproc_if!(r#if, {} {{\n\t{}\n}});",
            self.entity
                .descend_until_any(
                    &[
                        363, /* parenthesized_expression */
                        350, /* call_expression */
                        341, /* binary_expression */
                    ],
                    1
                )?
                .r()?,
            self.children
                .inclusive_offset(2, 1)
                .iter()
                .filter_map(|c| c.r())
                .map(|r| r.lines().collect::<Vec<&str>>().join("\n\t"))
                .collect::<Vec<String>>()
                .join("\n\t")
        ))
    }
}
