pub mod object;
use std::cell::RefCell;

use crate::evaluator::evaluator::get_null_object;
use object::RETURN_VALUE_OBJ;
pub use object::{Object, ObjectType};
pub mod boolean;
pub mod error;
pub mod integer;
pub mod null;
pub use boolean::Boolean;
pub use integer::Integer;
pub use null::Null;
pub use object::BOOLEAN_OBJ;
pub use object::INTEGER_OBJ;
pub use object::NULL_OBJ;

pub struct ReturnValue {
    pub value: RefCell<Box<dyn Object>>,
}

impl ReturnValue {
    pub fn new(value: Box<dyn Object>) -> Self {
        ReturnValue {
            value: RefCell::new(value),
        }
    }

    pub fn take_value(&self) -> Box<dyn Object> {
        let null = get_null_object();
        let mut borrowed = self.value.borrow_mut();
        std::mem::replace(&mut *borrowed, null)
    }
}

impl Object for ReturnValue {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn inspect(&self) -> String {
        self.value.borrow().inspect()
    }
    fn type_obj(&self) -> ObjectType {
        RETURN_VALUE_OBJ.to_string()
    }
}
