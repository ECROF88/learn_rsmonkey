use super::{Object, object::ERROR_OBJ};

pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }
    fn type_obj(&self) -> super::ObjectType {
        ERROR_OBJ.to_string()
    }
}
