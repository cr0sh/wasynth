use crate::instructions::Expression;

use super::GlobalType;

#[derive(Clone, Debug)]
pub struct Global {
    pub ty: GlobalType,
    pub init: Expression,
}
