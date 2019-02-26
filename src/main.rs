//!
//!
//!
//!
//!
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
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let dr: &Draw = if input.trim() == "b" {
        &boxed_greeting
    } else {
        &greeting
    };
    draw_text(dr);
}