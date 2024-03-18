
struct TEVariable {
    name: str
}
impl crate::nodes::expressions::te_expression::TEExpression for TEVariable{
    fn execute(&self) -> Box<str> {
        return Box::from("aa");
    }
}