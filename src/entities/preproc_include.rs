use crate::entities::{Entity, Entityable};

#[allow(unused)]
pub struct PreprocInclude<'a> {
    entity: &'a Entity<'a>,
}

impl<'a> Entityable<'a> for PreprocInclude<'a> {
    fn new(entity: &'a Entity<'a>) -> Self {
        Self { entity }
    }

    fn r(&'a self) -> Option<String> {
        let mut res = self
            .entity
            .descend_until_any(
                &[
                    172, /* system_lib_string */
                    371, /* string_literal */
                ],
                1,
            )?
            .r()?;

        res = res.strip_prefix("\"").unwrap_or(&res).into();
        res = res.strip_suffix("\"").unwrap_or(&res).into();
        res = res.split_terminator('.').next()?.into();

        Some(format!("preproc_include!({res});"))
    }
}
