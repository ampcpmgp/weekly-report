mod array;
mod borrowing;
mod data_types;
mod enum_case;
mod function;
mod guessing_game;
mod hash_maps;
mod if_let;
mod loop_case;
mod match_case;
mod module_scope;
mod ownership;
mod pointer;
mod slice_type;
mod strings;
mod struct_case;
mod struct_rectangle;
mod tuple;
mod vectors;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let name = args[1].as_str();

    match name {
        "array" => array::run(),
        "borrowing" => borrowing::run(),
        "data_types" => data_types::run(),
        "enum_case" => enum_case::run(),
        "function" => function::run(),
        "guessing_game" => guessing_game::run(),
        "hash_maps" => hash_maps::run(),
        "if_let" => if_let::run(),
        "loop_case" => loop_case::run(),
        "match_case" => match_case::run(),
        "module_scope" => crate::module_scope::run(),
        "ownership" => ownership::run(),
        "pointer" => pointer::run(),
        "slice_type" => slice_type::run(),
        "strings" => strings::run(),
        "struct_case" => struct_case::run(),
        "struct_rectangle" => struct_rectangle::run(),
        "tuple" => tuple::run(),
        "vectors" => vectors::run(),
        _ => println!("src配下にあるmod名を引数に入れてください"),
    }
}
