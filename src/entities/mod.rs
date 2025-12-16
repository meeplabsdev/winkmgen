use crate::entity::{Entity, pEntity};

mod argument_list;
mod assign;
mod assignment_expression;
mod binary_expression;
mod call_expression;
mod comment;
mod compound_statement;
mod condition_clause;
mod r#const;
mod declaration;
mod destructor_name;
mod expression_statement;
mod field_declaration;
mod field_declaration_list;
mod field_identifier;
mod function_declarator;
mod function_definition;
mod greater_than;
mod greater_than_or_equal;
mod identifier;
mod if_statement;
mod init_declarator;
mod linkage_specification;
mod number_literal;
mod parameter_declaration;
mod parameter_list;
mod parenthesized_expression;
mod pointer_declaration;
mod pointer_expression;
mod preproc_arg;
mod preproc_call;
mod preproc_def;
mod preproc_directive;
mod preproc_if;
mod preproc_ifdef;
mod preproc_include;
mod primitive_type;
mod string_content;
mod string_literal;
mod struct_specifier;
mod system_lib_string;
mod translation_unit;
mod type_definition;
mod type_identifier;
mod type_qualifier;

const NONE: Option<String> = Some(String::new());

pub trait Entityable<'a> {
    fn new(entity: pEntity<'a>) -> Self;
    fn r(&'a self) -> Option<String>;
}

impl<'a> Entity<'a> {
    pub fn r(&'a self) -> Option<String> {
        macro_rules! h {
            ( $m:ident::$c:ident ) => {{ $m::$c::new(self).r() }};
        }

        let mut res = match self.kind() {
            001 => h!(identifier::Identifier),
            018 => h!(preproc_arg::PreprocArg),
            019 => h!(preproc_directive::PreprocDirective),
            036 => h!(greater_than::GreaterThan),
            037 => h!(greater_than_or_equal::GreaterThanOrEqual),
            074 => h!(assign::Assign),
            082 => h!(r#const::Const),
            096 => h!(primitive_type::PrimitiveType),
            158 => h!(number_literal::NumberLiteral),
            170 => h!(string_content::StringContent),
            172 => h!(system_lib_string::SystemLibString),
            177 => h!(comment::Comment),
            219 => h!(translation_unit::TranslationUnit),
            222 => h!(preproc_include::PreprocInclude),
            223 => h!(preproc_def::PreprocDef),
            226 => h!(preproc_call::PreprocCall),
            227 => h!(preproc_if::PreprocIf),
            228 => h!(preproc_ifdef::PreprocIfdef),
            254 => h!(function_definition::FunctionDefinition),
            255 => h!(declaration::Declaration),
            256 => h!(type_definition::TypeDefinition),
            261 => h!(linkage_specification::LinkageSpecification),
            282 => h!(pointer_declaration::PointerDeclaration),
            286 => h!(function_declarator::FunctionDeclarator),
            294 => h!(init_declarator::InitDeclarator),
            295 => h!(compound_statement::CompoundStatement),
            297 => h!(type_qualifier::TypeQualifier),
            303 => h!(struct_specifier::StructSpecifier),
            305 => h!(field_declaration_list::FieldDeclarationList),
            307 => h!(field_declaration::FieldDeclaration),
            310 => h!(parameter_list::ParameterList),
            311 => h!(parameter_declaration::ParameterDeclaration),
            317 => h!(expression_statement::ExpressionStatement),
            318 => h!(if_statement::IfStatement),
            338 => h!(assignment_expression::AssignmentExpression),
            339 => h!(pointer_expression::PointerExpression),
            341 => h!(binary_expression::BinaryExpression),
            350 => h!(call_expression::CallExpression),
            360 => h!(argument_list::ArgumentList),
            363 => h!(parenthesized_expression::ParenthesizedExpression),
            371 => h!(string_literal::StringLiteral),
            441 => h!(condition_clause::ConditionClause),
            479 => h!(destructor_name::DestructorName),
            538 => h!(field_identifier::FieldIdentifier),
            542 => h!(type_identifier::TypeIdentifier),
            005 | // (
            007 | // ,
            008 | // )
            010 | // \n
            042 | // ;
            065 | // {
            066 | // }
            265   // md_declspec_modifier
                  => NONE,
            65535 => panic!("Failed to parse token!"),
            _ => None,
        };

        if res.is_none() {
            println!("got None for {:?}", self.kind());
        }

        if res == NONE {
            res = None;
        }

        res
    }

    // pub fn ascend_until(&self, kind: u16) -> Option<Node<'a>> {
    //     let mut node = self.node;

    //     while node.kind_id() != kind {
    //         node = node.parent()?;
    //     }

    //     Some(node)
    // }

    // pub fn ascend_until_any(&'a self, kind: &[u16]) -> Option<Node<'a>> {
    //     for k in kind {
    //         match self.ascend_until(*k) {
    //             Some(e) => return Some(e),
    //             None => {}
    //         }
    //     }

    //     None
    // }

    // pub fn descend_until(&'a self, kind: u16, max_depth: u32) -> Option<&'a Entity<'a>> {
    //     fn descend<'a>(
    //         entity: &'a Entity<'a>,
    //         kind: u16,
    //         depth: u32,
    //         max_depth: u32,
    //     ) -> Option<&'a Entity<'a>> {
    //         if depth > max_depth {
    //             return None;
    //         }

    //         if entity.kind == kind {
    //             return Some(entity);
    //         }

    //         for child in &entity.children {
    //             match descend(child, kind, depth + 1, max_depth) {
    //                 Some(e) => return Some(e),
    //                 None => {}
    //             }
    //         }

    //         None
    //     }

    //     descend(self, kind, 0, max_depth)
    // }

    // pub fn descend_until_any(&'a self, kind: &[u16], max_depth: u32) -> Option<&'a Entity<'a>> {
    //     for k in kind {
    //         match self.descend_until(*k, max_depth) {
    //             Some(e) => return Some(e),
    //             None => {}
    //         }
    //     }

    //     None
    // }
}
