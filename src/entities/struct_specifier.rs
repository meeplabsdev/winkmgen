use crate::entities::{Entity, EntityList, Entityable};

#[allow(unused)]
pub struct StructSpecifier<'a> {
    entity: &'a Entity<'a>,
    children: &'a EntityList<'a>,
}

impl<'a> Entityable<'a> for StructSpecifier<'a> {
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

        let field_declaration_list = self
            .entity
            .descend_until(305 /* field_declaration_list */, 1)?;

        let struct_name = self
            .entity
            .descend_until(542 /* type_identifier */, 1)?
            .r()?;

        let mut functions = Vec::new();
        for f in field_declaration_list
            .children
            .iter()
            .filter(|c| c.kind == 254)
        {
            if f.descend_until(286 /* function_declarator */, 1)?
                .descend_until(1 /* identifier */, 1)?
                .r()?
                == String::from("drop")
            {
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
            field_declaration_list.r()?,
            functions.join("\n\n")
        ))
    }
}
