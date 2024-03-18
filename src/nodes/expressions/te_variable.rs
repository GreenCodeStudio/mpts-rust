
pub struct TEVariable {
    pub(crate) name: Box<str>
}
impl crate::nodes::expressions::te_expression::TEExpression for TEVariable{
    fn execute(&self) -> Box<str> {
        return Box::from("aa");
    }

    fn get_type(&self) -> Box<str> {
        return Box::from("TEVariable");
    }
}