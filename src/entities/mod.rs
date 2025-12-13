use tree_sitter::{Node, TreeCursor};

use crate::get_from_range;

mod argument_list;
mod assign;
mod assignment_expression;
mod binary_expression;
mod call_expression;
mod close_context;
mod comment;
mod compound_statement;
mod destructor_name;
mod expression_statement;
mod field_declaration;
mod field_declaration_list;
mod field_identifier;
mod function_declarator;
mod function_definition;
mod greater_than_or_equal;
mod identifier;
mod if_statement;
mod open_context;
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

pub trait ToRust<'a> {
    fn r(&'a self) -> Option<String>;
}

#[derive(Debug, Clone)]
pub struct Entity<'a> {
    node: Node<'a>,
    content: String,
    children: Vec<Entity<'a>>,
}

impl<'a> Entity<'a> {
    pub fn new(cursor: &mut TreeCursor<'a>, node: Node<'a>) -> Self {
        Self {
            node,
            content: get_from_range(node.byte_range()),
            children: node
                .children(cursor)
                .collect::<Vec<Node>>()
                .iter()
                .map(|c| Entity::new(cursor, *c))
                .collect(),
        }
    }

    pub fn ascend_until(&self, kind: u16) -> Option<Node<'a>> {
        let mut node = self.node;
        while node.kind_id() != kind {
            node = node.parent()?;
        }

        Some(node)
    }
}

impl<'a> ToRust<'a> for Entity<'a> {
    fn r(&'a self) -> Option<String> {
        macro_rules! h {
            ( $x:expr ) => {{ $x(self).r() }};
        }

        let res = match self.node.kind_id() {
            1 => h!(identifier::Identifier),
            18 => h!(preproc_arg::PreprocArg),
            19 => h!(preproc_directive::PreprocDirective),
            37 => h!(greater_than_or_equal::GreaterThanOrEqual),
            65 => h!(open_context::OpenContext),
            66 => h!(close_context::CloseContext),
            74 => h!(assign::Assign),
            96 => h!(primitive_type::PrimitiveType),
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
            256 => h!(type_definition::TypeDefinition),
            282 => h!(pointer_declaration::PointerDeclaration),
            286 => h!(function_declarator::FunctionDeclarator),
            295 => h!(compound_statement::CompoundStatement),
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
            479 => h!(destructor_name::DestructorName),
            538 => h!(field_identifier::FieldIdentifier),
            542 => h!(type_identifier::TypeIdentifier),
            _ => None,
        };

        if res.is_none() {
            println!("got None for {:?}", self.node.kind());
        }

        res
    }
}
