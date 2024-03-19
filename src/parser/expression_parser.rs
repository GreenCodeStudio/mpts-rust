/*
import {TDocumentFragment} from "../nodes/TDocumentFragment";
import {TEVariable} from "../nodes/expressions/TEVariable";
import {TEBoolean} from "../nodes/expressions/TEBoolean";
import {TENumber} from "../nodes/expressions/TENumber";
import {TEString} from "../nodes/expressions/TEString";
import {TEEqual} from "../nodes/expressions/TEEqual";
import {AbstractParser} from "./AbstractParser";
import {TEProperty} from "../nodes/expressions/TEProperty";
import {TEMethodCall} from "../nodes/expressions/TEMethodCall";
import {TEConcatenate} from "../nodes/expressions/TEConcatenate";
import {TEAdd} from "../nodes/expressions/TEAdd";
import {TESubtract} from "../nodes/expressions/TESubtract";
import {TEOrNull} from "../nodes/expressions/TEOrNull";

export class ExpressionParser extends AbstractParser {
    constructor(text) {
        super();
        this.text = text;
        this.position = 0;
    }

    static Parse(text, endLevel=0) {
        return (new ExpressionParser(text)).parse_normal(endLevel);
    }

    parse_normal(endLevel = 0) {
        let lastNode = null;
        while (this.position < this.text.length) {
            const char = this.text[this.position];
            if (/\s/.test(char)) {
                this.position++;
            } else if (lastNode && char == '.') {
                this.position++;
                let name = this.readUntill(/['"\(\)=\.:\s>+\-*?]/);
                lastNode = new TEProperty(lastNode, name);
            } else if (/[0-9\.]/.test(char)) {
                this.position++;
                let value = char+this.readUntill(/[^0-9\.e]/);
                lastNode = new TENumber(+value);
            } else if (char == '"') {
                this.position++;
                let value = this.readUntill(/"/);
                this.position++;
                lastNode = new TEString(value);
            } else if (char == "'") {
                this.position++;
                let value = this.readUntill(/'/);
                this.position++;
                lastNode = new TEString(value);
            } else if (char == "(") {
                if (lastNode) {
                    lastNode = new TEMethodCall(lastNode);
                    this.position++;
                    this.skipWhitespace();
                    while (this.text[this.position] != ')') {
                        if (this.position >= this.text.length) throw new Error('Unexpected end of input');

                        let value = this.parse_normal(2);
                        lastNode.args.push(value);
                        if(this.text[this.position] ==',')
                            this.position++;
                    }
                    this.position++;
                } else {
                    this.position++;
                    let value = this.parse_normal(1);
                    this.position++;
                    lastNode = value;
                }
            } else if (char == ")") {
                if (endLevel >= 1) {
                    break;
                } else {
                    throw new Error("( not opened");
                }
            } else if (char == "=" && this.text[this.position + 1] == "=") {
                this.position += 2;
                let right = this.parse_normal(2);
                lastNode = new TEEqual(lastNode, right);
            }  else if (char == "?"&& this.text[this.position + 1] == "?") {
                if (endLevel >= 5) {
                    break;
                }
                this.position+=2;
                let right = this.parse_normal(5);
                lastNode = new TEOrNull(lastNode, right);
            } else if (char == ",") {
                if (endLevel >= 2) {
                    break;
                }
                else {
                    throw new Error("Unexpected character");
                }
            }else if (char == "+") {
                if (endLevel >= 4) {
                    break;
                }
                this.position++;
                let right = this.parse_normal(4);
                lastNode = new TEAdd(lastNode, right);
            } else if (char == "-") {
                if (endLevel >= 4) {
                    break;
                }
                this.position++;
                let right = this.parse_normal(4);
                lastNode = new TESubtract(lastNode, right);
            } else if (char == ":") {
                this.position++;
                let right = this.parse_normal(3);
                lastNode = new TEConcatenate(lastNode, right);
            } else if (char == ">" || char == "\\") {
                if (lastNode) {
                    break
                } else {
                    throw new Error("Unexpected character");
                }
            } else {
                if (lastNode) {
                    break;
                }
                let name = this.readUntill(/['"\(\)=\.\s:>/+\-*?,]/);
                if (name == 'true')
                    lastNode = new TEBoolean(true)
                else if (name == 'false')
                    lastNode = new TEBoolean(false)
                else
                    lastNode = new TEVariable(name);
            }
        }
        return lastNode;
    }

}

 */
use crate::nodes::expressions::te_boolean::TEBoolean;
use crate::nodes::expressions::te_expression::TEExpression;
use crate::nodes::expressions::te_variable::TEVariable;

pub struct ExpressionParser {
    text: Box<str>,
    position: u64,
}

impl ExpressionParser {
    pub fn read_untill(&mut self, regexp: Box<str>) -> Box<str> {
        let mut ret = String::new();
        while self.position < self.text.len() as u64 {
            let char = self.text.chars().nth(self.position as usize).unwrap();
            if regexp.contains(char) {
                break;
            }
            ret.push(char);
            self.position += 1;
        }
        return Box::from(ret);
    }

    fn skip_whitespace(&mut self) {
        self.read_untill(Box::from(r"\S"));
    }

    // fn read_untill_text(&mut self, text: Box<str>) -> Box<str> {
    //     let mut ret = Box::from("");
    //     while self.position < self.text.len() as u64 {
    //         let char = self.text.chars().nth(self.position as usize).unwrap();
    //         if self.text[self.position as usize..].starts_with(&text) {
    //             break;
    //         }
    //         ret.push(char);
    //         self.position += 1;
    //     }
    //     return ret;
    // }

    // fn throw(&self, message: Box<str>) {
    //     let lines = self.text[..self.position as usize].split('\n');
    //     panic!("MptsParserError: {} at line {}", message, lines.len());
    // }

    pub fn parse(text: Box<str>, end_level: u64) ->Option<Box<dyn TEExpression>> {
        let mut parser = ExpressionParser { text: text, position: 0 };
        return parser.parse_normal(end_level);
    }
    pub fn parse_normal(&mut self, end_level: u64) -> Option<Box<dyn TEExpression>> {
        //return Box::new(crate::nodes::expressions::te_variable::TEVariable { name: Box::from("tmp") });
        let mut lastNode: Option<Box<dyn TEExpression>> = None;
        while (self.position < self.text.len() as u64) {
            let char: char = self.text.chars().nth(self.position as usize).unwrap();
            if char.is_whitespace() {
                self.position += 1;
            } else {
                if (lastNode.is_some()) {
                    break;
                }
                let name = self.read_untill(Box::from("['\"\\(\\)=\\.:\\s>\\+\\-*?]"));
                if name == Box::from("true") {
                    lastNode = Option::Some(Box::new(TEBoolean { value: true }));
                } else if name == Box::from("false") {
                    lastNode = Option::Some(Box::new(TEBoolean { value: false }));
                } else {
                    lastNode = Option::Some(Box::new(TEVariable { name: name }));
                }
            }
        }
        return lastNode;
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::nodes::expressions::te_variable::TEVariable;
    use super::*;

    #[test]
    fn variable() {
        let obj = ExpressionParser::parse(Box::from("var1"), 0);
        assert!(obj.is_some());
        //assert_eq!(obj.get_type().deref(), "TEVariable");
        assert_eq!(obj.unwrap().get_type().deref(), "TEVariable");
    }
}
