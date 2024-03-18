pub trait TEExpression {
    fn execute(&self) -> Box<str>;
    fn get_type(&self) ->Box<str>;
}