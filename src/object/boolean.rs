use super::{Object, object::BOOLEAN_OBJ};

#[derive(Debug, Clone, Copy)]
pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
    fn type_obj(&self) -> super::ObjectType {
        BOOLEAN_OBJ.to_string()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_object(&self) -> Box<dyn Object> {
        Box::new(Self { value: self.value })
    }
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Boolean { value }
    }
}
