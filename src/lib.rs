use std::sync::OnceLock;

type Group = [&'static str; 4];
type Layout = [Group; 4];

fn choices_text(a: &'static str, b: &'static str, c: &'static str, d: &'static str) -> Choice<'static> {
    Choices::new(
        Choice::Text(a),
        Choice::Text(b),
        Choice::Text(c),
        Choice::Text(d),
    ).into()
}

fn toplevel() -> &'static Choices<'static> {
    static MENU: OnceLock<Choices> = OnceLock::new();
    MENU.get_or_init(|| {
        let abcd = choices_text("a", "b", "c", "d");
        let efgh = choices_text("e", "f", "g", "h");
        let ijkl = choices_text("i", "j", "k", "l");
        let mnop = choices_text("m", "n", "o", "p");
        let alpha1 = Choice::Menu(Choices::new(abcd, efgh, ijkl, mnop));

        let qrst = choices_text("q", "r", "s", "t");
        let uvwx = choices_text("u", "v", "w", "x");
        let yzdc = choices_text("y", "z", ".", ",");
        let qbsd = choices_text("?", "!", "'", "\"");
        let alpha2 = Choice::Menu(Choices::new(qrst, uvwx, yzdc, qbsd));

        let num1 = choices_text("1", "2", "3", "4");
        let num2  =choices_text("5", "6", "7", "8");
        let num3 = choices_text("9", "0", "+", "-");
        let num4 = choices_text("~", "$", "%", "#");
        let numbers = Choice::Menu(Choices::new(num1, num2, num3, num4));

        Choices::new(alpha1, alpha2, Choice::None, numbers)
    })
}

static LAYOUTS: [Layout; 3] = [
    [
        ["a", "b", "c", "d"],
        ["e", "f", "g", "h"],
        ["i", "j", "k", "l"],
        ["m", "n", "o", "p"],
    ],

    [
        ["q", "r", "s", "t"],
        ["u", "v", "w", "x"],
        ["y", "z", ".", ","],
        ["?", "!", "'", "\""],
    ],

    [
        ["1", "2", "3", "4"],
        ["5", "6", "7", "8"],
        ["9", "0", "+", "-"],
        ["~", "$", "%", "#"],
    ],
];

#[derive(Clone)]
enum Choice<'a> {
    None,
    Menu(Choices<'a>),
    Text(&'a str),
}

#[derive(Clone)]
pub struct Choices<'a> {
    label: String,
    a: Choice<'a>,
    b: Choice<'a>,
    c: Choice<'a>,
    d: Choice<'a>,
}

impl<'a> From<Choices<'a>> for Choice<'a> {
    fn from(item: Choices<'a>) -> Choice<'a> {
        Choice::Menu(&item)
    }
}

impl<'a> Choices<'a> {
    pub fn new(a: Choice<'a>, b: Choice<'a>, c: Choice<'a>, d: Choice<'a>) -> Self {
        let label = String::new();
        Self { label, a, b, c, d }
    }

    pub fn set_label(&mut self, text: String) -> &Self {
        self.label.push_str(&text);
        self
    }
}

pub struct Interface<'a> {
    //label: &'a str,
    buffer: String,
    //toplevel_choices: Choices<'a>,
    choices: &'a Choices<'a>,
}

impl<'a> Interface<'a> {
    pub fn new(choices: &'a Choices<'a>) -> Interface<'a> {
        let buffer = String::new();
        //let toplevel_choices = choices.clone();
        //let choices = &choices;
        Self { buffer: buffer, /*toplevel_choices,*/ choices: choices }
    }

    pub fn text(&self) -> String {
        self.buffer.clone()
    }

    fn append(&'a mut self, text: &'a str) -> &'a Self {
        self.buffer.push_str(text);
        self
    }

    fn select(&'a mut self, choice: &Choice<'a>) {
        match choice {
            Choice::Menu(menu) => { self.choices = menu; },
            Choice::Text(text) => { self.append(text); },
            Choice::None => (),
        };
    }

    pub fn up(&'a mut self) {
        self.select(&self.choices.a)
    }

    pub fn right(&'a mut self) {
        self.select(&self.choices.b)
    }

    pub fn down(&'a mut self) {
        self.select(&self.choices.c)
    }

    pub fn left(&'a mut self) {
        self.select(&self.choices.d)
    }
}


#[cfg(test)]
mod tests {
    use crate::Interface;
/*
    #[test]
    fn hello_world() {
        let iface = Interface::new()
            .a().b().d()
            .a().b().a()
            .a().c().d()
            .a().c().d()
            .a().d().c()
            .b().c().d()
            .space()
            .space()
            .delete()
            .b().b().c()
            .a().d().c()
            .b().a().b()
            .a().c().d()
            .a().a().d()
            .b().d().b();
        assert_eq!("hello, world!", iface.text());
    }
*/
}
