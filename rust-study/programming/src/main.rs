mod array;
mod borrowing;
mod box_type;
mod closure;
mod data_types;
mod deref;
mod drop;
mod enum_case;
mod function;
mod generics_syntax;
mod guessing_game;
mod hash_maps;
mod if_let;
mod iterators;
mod lifetime;
mod loop_case;
mod match_case;
mod module_scope;
mod ownership;
mod panic_or_not;
mod performance;
mod pointer;
mod rc;
mod recoverable_errors;
mod slice_type;
mod strings;
mod struct_case;
mod struct_rectangle;
mod traits_syntax;
mod tuple;
mod unrecoverable_errors;
mod vectors;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let name = args[1].as_str();

    match name {
        "array" => array::run(),
        "borrowing" => borrowing::run(),
        "box_type" => box_type::run(),
        "closure" => closure::run(),
        "data_types" => data_types::run(),
        "deref" => deref::run(),
        "drop" => drop::run(),
        "enum_case" => enum_case::run(),
        "function" => function::run(),
        "generics_syntax" => generics_syntax::run(),
        "guessing_game" => guessing_game::run(),
        "hash_maps" => hash_maps::run(),
        "if_let" => if_let::run(),
        "iterators" => iterators::run(),
        "lifetime" => lifetime::run(),
        "loop_case" => loop_case::run(),
        "match_case" => match_case::run(),
        "module_scope" => crate::module_scope::run(),
        "ownership" => ownership::run(),
        "panic_or_not" => panic_or_not::run(),
        "performance" => performance::run(),
        "pointer" => pointer::run(),
        "rc" => rc::run(),
        "recoverable_errors" => recoverable_errors::run(),
        "slice_type" => slice_type::run(),
        "strings" => strings::run(),
        "struct_case" => struct_case::run(),
        "struct_rectangle" => struct_rectangle::run(),
        "traits_syntax" => traits_syntax::run(),
        "tuple" => tuple::run(),
        "unrecoverable_errors" => unrecoverable_errors::run(),
        "vectors" => vectors::run(),
        _ => println!("src配下にあるmod名を引数に入れてください"),
    }
}
