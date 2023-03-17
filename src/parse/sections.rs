mod code;
mod custom;
mod data;
mod data_count;
mod element;
mod export;
mod function;
mod global;
mod import;
mod memory;
mod name;
mod start;
mod table;
mod r#type;

pub use {
    code::*, custom::*, data::*, data_count::*, element::*, export::*, function::*, global::*,
    import::*, memory::*, name::*, r#type::*, start::*, table::*,
};
