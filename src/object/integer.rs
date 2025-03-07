use super::{Object, ObjectType, object::INTEGER_OBJ};

#[derive(Debug)]
pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
    fn type_obj(&self) -> ObjectType {
        INTEGER_OBJ.to_string()
    }
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Integer { value }
    }
}
