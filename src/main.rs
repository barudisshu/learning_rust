#[macro_use]
extern crate t_bang;

use std::io::Read;
use std::io::Write;

use t_bang::*;

trait Draw {
    fn draw(&self);
}

struct Text { characters: String }

impl Text {
    fn from(text: &str) -> Text {
        Text { characters: text.to_string() }
    }
}

impl Draw for Text {
    fn draw(&self) {
        print!("{}", self.characters);
    }
}

struct BoxedText {
    text: Text,
    first: char,
    last: char,
}

impl BoxedText {
    fn with_text_and_borders(
        text: &str, first: char, last: char) -> BoxedText {
        BoxedText {
            text: Text::from(text),
            first: first,
            last: last,
        }
    }
}

impl Draw for BoxedText {
    fn draw(&self) {
        print!("{}", self.first);
        self.text.draw();
        print!("{}", self.last);
    }
}

fn draw_text(txt: &Draw) {
    txt.draw();
}

fn main() {
    let greeting = Text::from("Hello");
    let boxed_greeting =
        BoxedText::with_text_and_borders("Hi", '[', ']');
    draw_text(&greeting);
    print!(", ");
    draw_text(&boxed_greeting);
}