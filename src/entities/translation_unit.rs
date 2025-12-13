use crate::entities::{Entity, ToRust};

#[allow(unused)]
pub struct TranslationUnit<'a>(pub &'a Entity<'a>);
impl<'a> ToRust<'a> for TranslationUnit<'a> {
    fn r(&'a self) -> Option<String> {
        Some(format!(
            "#![allow(nonstandard_style, unused)]\n\n{}\n",
            self.0
                .children
                .iter()
                .filter_map(|c| c.r())
                .collect::<Vec<String>>()
                .join("\n\n")
        ))
    }
}
