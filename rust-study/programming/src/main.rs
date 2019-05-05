mod array;
mod borrowing;
mod data_types;
mod enum_case;
mod function;
mod if_let;
mod loop_case;
mod match_case;
mod ownership;
mod pointer;
mod slice_type;
mod struct_case;
mod struct_rectangle;
mod tuple;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let name = args[1].as_str();

    match name {
        "array" => array::run(),
        "borrowing" => borrowing::run(),
        "data_types" => data_types::run(),
        "enum_case" => enum_case::run(),
        "function" => function::run(),
        "if_let" => if_let::run(),
        "loop_case" => loop_case::run(),
        "match_case" => match_case::run(),
        "ownership" => ownership::run(),
        "pointer" => pointer::run(),
        "slice_type" => slice_type::run(),
        "struct_case" => struct_case::run(),
        "struct_rectangle" => struct_rectangle::run(),
        "tuple" => tuple::run(),
        _ => println!("src配下にあるmod名を引数に入れてください"),
    }
}
