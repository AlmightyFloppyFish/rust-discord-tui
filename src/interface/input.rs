use std::sync::RwLock;

#[derive(Eq, PartialEq)]
pub enum Mode {
    Exec,
    Insert,
}

pub struct ModeSwitcher {
    mode: RwLock<Mode>,

    // There contain options for performance reasons, i take, change, replace.
    // This prevents any heap copying
    pub insert_buffer: RwLock<String>,
    pub exec_buffer: RwLock<String>,
}

impl ModeSwitcher {
    pub fn new(text: &str) -> Self {
        ModeSwitcher {
            mode: RwLock::new(Mode::Insert),
            insert_buffer: RwLock::new(String::from(text)),
            exec_buffer: RwLock::new(String::new()),
        }
    }

    pub fn is_exec(&self) -> bool {
        *self.mode.read().unwrap() == Mode::Exec
    }
    pub fn is_insert(&self) -> bool {
        *self.mode.read().unwrap() == Mode::Insert
    }

    pub fn set_mode(&self, new: Mode) {
        *self.mode.write().unwrap() = new;
    }

    pub fn is_escaping(&self) -> bool {
        (*self.insert_buffer.read().unwrap())
            .chars()
            .last()
            .or(Some('a'))
            .unwrap()
            == ':'
    }
    // Inner mutability, self ref does not need to be mutable
    pub fn escape(&self) {
        *self.mode.write().unwrap() = Mode::Exec;
        // Remove leftover ':' char created by double escape
        self.insert_buffer.write().unwrap().pop();
    }
}
