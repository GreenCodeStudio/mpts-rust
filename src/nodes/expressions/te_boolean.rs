use crate::nodes::expressions::te_expression::TEExpression;

pub struct TEBoolean {
    pub value: bool
}
impl TEExpression for TEBoolean{
    fn execute(&self) -> Box<str> {
        return Box::from("aa");
    }

    fn get_type(&self) -> Box<str> {
        return Box::from("TEBoolean");
    }
}