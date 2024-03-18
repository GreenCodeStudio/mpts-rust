pub(crate) trait TEExpression {
    fn execute(&self) -> Box<str>;
}