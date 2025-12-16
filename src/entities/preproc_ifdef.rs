use crate::entities::{Entity, EntityList, Entityable};

#[allow(unused)]
pub struct PreprocIfdef<'a> {
    entity: &'a Entity<'a>,
    children: &'a EntityList<'a>,
}

impl<'a> Entityable<'a> for PreprocIfdef<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self {
            entity,
            children: &entity.children,
        }
    }

    fn r(&'a self) -> Option<String> {
        if self.children.index(1)?.kind != 1 {
            return None;
        }

        Some(format!(
            "preproc_if!(r#{}, {}, {{\n\t{}\n}});",
            match self.children.index(0)?.kind {
                13 => "ifndef",
                _ => return None,
            },
            self.children.index(1)?.r()?,
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
