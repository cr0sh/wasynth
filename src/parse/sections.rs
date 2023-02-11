pub mod code;
pub mod custom;
pub mod data;
pub mod data_count;
pub mod element;
pub mod export;
pub mod function;
pub mod global;
pub mod import;
pub mod memory;
pub mod start;
pub mod table;
pub mod r#type;

pub use {
    code::*, custom::*, data::*, data_count::*, element::*, export::*, function::*, global::*,
    import::*, memory::*, r#type::*, start::*, table::*,
};
