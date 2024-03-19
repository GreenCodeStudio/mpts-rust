/*
import {MptsParserError} from "./MptsParserError";

export class AbstractParser {

    readUntill(regexp) {
        let ret = '';
        while (this.position < this.text.length) {
            const char = this.text[this.position];
            if (regexp.test(char))
                break;
            ret += char;
            this.position++;
        }
        return ret;
    }

    skipWhitespace() {
        this.readUntill(/\S/)
    }

    readUntillText(text) {
        let ret = '';
        while (this.position < this.text.length) {
            const char = this.text[this.position];
            if (this.text.substr(this.position, text.length) == text)
                break;
            ret += char;
            this.position++;
        }
        return ret;
    }

    throw(message) {
        let lines = this.text.substr(0, this.position).split('\n');
        throw new MptsParserError(message, lines.length, lines[lines.length - 1].length, this.text.substr(this.position, 10))
    }
}
 */
pub struct AbstractParser{
    text: Box<str>,
    position: u64,
}
impl AbstractParser{
// pub fn read_untill(&mut self, regexp: Box<str>) -> String {
//         let mut ret: String = String::new();
//         while self.position < self.text.len() as u64 {
//             let char = self.text.chars().nth(self.position as usize).unwrap();
//             if regexp.contains(char) {
//                 break;
//             }
//             ret.push(char);
//             self.position += 1;
//         }
//         return ret;
//     }
//
//     fn skip_whitespace(&mut self) {
//         self.read_untill(Box::from(r"\S"));
//     }
//
//     fn read_untill_text(&mut self, text: Box<str>) -> Box<str> {
//         let mut ret = Box::from("");
//         while self.position < self.text.len() as u64 {
//             let char = self.text.chars().nth(self.position as usize).unwrap();
//             if self.text[self.position as usize..].starts_with(&text) {
//                 break;
//             }
//             ret.push(char);
//             self.position += 1;
//         }
//         return ret;
//     }
//
//     fn throw(&self, message: Box<str>) {
//         let lines = self.text[..self.position as usize].split('\n');
//         panic!("MptsParserError: {} at line {}", message, lines.len());
//     }
}
