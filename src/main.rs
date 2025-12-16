use std::{
    fs,
    io::{Read, Write as _},
    ops::Range,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};
use tree_sitter::{Parser, TreeCursor};

use crate::{
    entity::{Entity, EntityArena},
    error::Error,
};

pub mod entities;
pub mod entity;
pub mod error;
pub mod windowskits;

static SOURCE: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(Vec::new()));

fn main() -> Result<(), Error> {
    let out_file = PathBuf::from("out.rs");
    let in_file = windowskits::get_test_header_path();
    // let in_file = windowskits::get_header_path("fltsafe.h");

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_cpp::LANGUAGE.into())
        .expect("Error loading grammar");

    let mut source_code = String::new();
    fs::File::open(in_file)?.read_to_string(&mut source_code)?;
    source_code = parse_source(source_code);

    SOURCE.lock().unwrap().extend(source_code.as_bytes());
    let tree = parser.parse(source_code, None).unwrap();
    enumerate_tree(&mut tree.walk(), 0);

    let arena = EntityArena::new();
    let mut cursor = tree.walk();
    let root = dive(&arena, &mut cursor);

    fn dive<'a>(arena: &'a EntityArena<'a>, cursor: &mut TreeCursor<'a>) -> &'a Entity<'a> {
        let node = cursor.node();
        let mut children: Vec<&'a Entity<'a>> = Vec::new();

        if cursor.goto_first_child() {
            children.push(dive(arena, cursor));
            while cursor.goto_next_sibling() {
                children.push(dive(arena, cursor));
            }
            cursor.goto_parent();
        }

        arena.push(Entity::new(node, children))
    }

    let mut file = fs::File::create(out_file)?;
    file.write_all(root.r().ok_or("Invalid file")?.as_bytes())?;

    Ok(())
}

fn enumerate_tree(cursor: &mut TreeCursor, depth: usize) {
    let node = cursor.node();
    if ![177].contains(&node.kind_id()) {
        println!(
            "{}{:?} ({})",
            "|   ".repeat(depth),
            node.kind(),
            node.kind_id()
        );
    }

    if cursor.goto_first_child() {
        if depth < 1 {
            enumerate_tree(cursor, depth + 1);
            while cursor.goto_next_sibling() {
                enumerate_tree(cursor, depth + 1);
            }
        } else {
            enumerate_tree(cursor, depth + 1);
            while cursor.goto_next_sibling() {
                enumerate_tree(cursor, depth + 1);
            }
        }

        cursor.goto_parent();
    }

    while cursor.goto_next_sibling() {
        enumerate_tree(cursor, depth);
    }
}

fn get_from_range(range: Range<usize>) -> String {
    String::from_utf8(SOURCE.lock().unwrap()[range.start..range.end].to_vec()).unwrap()
}

fn parse_source(source: String) -> String {
    source
        .replace("EXTERN_C ", "extern \"C\" ")
        .replace("DECLSPEC_SELECTANY ", "__declspec(selectany) ")
}
