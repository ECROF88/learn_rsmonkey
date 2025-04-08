pub type ObjectType = String;

pub trait Object: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn type_obj(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn clone_object(&self) -> Box<dyn Object>;
}
pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NULL_OBJ: &str = "NULL";
pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
pub const ERROR_OBJ: &str = "ERROR";
