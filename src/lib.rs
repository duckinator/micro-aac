type Group = [&'static str; 4];
type Layout = [Group; 4];

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
struct State {
    text: String,
}
impl State {
    pub fn new() -> Self {
        Self { text: String::new() }
    }

    fn append(&self, new_text: &'static str) -> Self {
        let mut state = self.clone();
        state.text.push_str(new_text);
        state
    }

    fn space(&self) -> Self {
        self.append(" ")
    }

    fn delete(&self) -> Self {
        let mut state = self.clone();
        state.text.pop();
        state
    }
}

pub struct GroupIface {
    group: Group,
    state: State,
}
impl GroupIface {
    pub fn a(&self) -> Interface {
        Interface { state: self.state.append(self.group[0]) }
    }
    pub fn b(&self) -> Interface {
        Interface{ state: self.state.append(self.group[1]) }
    }
    pub fn c(&self) -> Interface {
        Interface { state: self.state.append(self.group[2]) }
    }
    pub fn d(&self) -> Interface {
        Interface{ state: self.state.append(self.group[3]) }
    }
}


pub struct LayoutIface {
    layout: Layout,
    state: State,
}
impl LayoutIface {
    pub fn a(&self) -> GroupIface {
        GroupIface { group: self.layout[0], state: self.state.clone() }
    }

    pub fn b(&self) -> GroupIface {
        GroupIface { group: self.layout[1], state: self.state.clone() }
    }

    pub fn c(&self) -> GroupIface {
        GroupIface { group: self.layout[2], state: self.state.clone() }
    }

    pub fn d(&self) -> GroupIface {
        GroupIface { group: self.layout[3], state: self.state.clone() }
    }
}

pub struct Interface {
    state: State,
}
impl Interface {
    pub fn new() -> Self {
        Self { state: State::new() }
    }

    pub fn text(&self) -> String {
        self.state.clone().text
    }

    pub fn clear(&self) -> Interface {
        Self::new()
    }

    pub fn space(&self) -> Self {
        Self { state: self.state.space() }
    }

    pub fn delete(&self) -> Self {
        Self { state: self.state.delete() }
    }

    pub fn a(&self) -> LayoutIface {
        LayoutIface { layout: LAYOUTS[0], state: self.state.clone() }
    }

    pub fn b(&self) -> LayoutIface {
        LayoutIface { layout: LAYOUTS[1], state: self.state.clone() }
    }

    pub fn c(&self) -> LayoutIface {
        LayoutIface { layout: LAYOUTS[2], state: self.state.clone() }
    }
}

#[cfg(test)]
mod tests {
    use crate::Interface;

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
}
