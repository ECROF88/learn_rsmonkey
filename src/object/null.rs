use super::{Object, object::NULL_OBJ};

#[derive(Debug, Clone)]
pub struct Null {}

impl Object for Null {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }
    fn inspect(&self) -> String {
        "null".to_string()
    }
    fn type_obj(&self) -> super::ObjectType {
        NULL_OBJ.to_string()
    }
}
