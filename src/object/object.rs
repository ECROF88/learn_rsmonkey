pub type ObjectType = String;

pub trait Object: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn type_obj(&self) -> ObjectType;
    fn inspect(&self) -> String;
}
pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NULL_OBJ: &str = "NULL";
