#[derive(Default)]
pub enum Focus {
    #[default]
    Username,
    Password,
    Programms,
}

#[derive(Default)]
pub struct TextField {
    value: String,
    cursor_position: usize,
}

impl TextField {
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let chars = self.value.chars();
        self.value = chars
            .clone()
            .take(self.cursor_position)
            .chain([new_char])
            .chain(chars.skip(self.cursor_position))
            .collect();

        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        if self.cursor_position == 0 {
            return;
        }

        let chars = self.value.chars();
        let before_char_to_delete = chars.clone().take(self.cursor_position - 1);
        let after_char_to_delete = self.value.chars().skip(self.cursor_position);

        self.value = before_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor_left();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.value.len())
    }

    fn reset(&mut self) {
        self.cursor_position = 0;
        self.value = String::default();
    }
}

#[derive(Default)]
pub struct App {
    username: TextField,
    password: TextField,
    programms: Vec<String>,
    programm_idx: usize,
    focus: Focus,
}

impl App {
    pub fn focus(&self) -> &Focus {
        &self.focus
    }

    pub fn username(&self) -> &str {
        &self.username.value
    }

    pub fn password(&self) -> &str {
        &self.password.value
    }

    pub fn next_input(&mut self) {
        match self.focus {
            Focus::Username => self.focus = Focus::Password,
            Focus::Password => self.focus = Focus::Programms,
            Focus::Programms => {}
        }
    }

    pub fn prev_input(&mut self) {
        match self.focus {
            Focus::Username => {}
            Focus::Password => self.focus = Focus::Username,
            Focus::Programms => self.focus = Focus::Password,
        }
    }

    pub fn enter_char(&mut self, to_insert: char) {
        match self.focus {
            Focus::Username => self.username.enter_char(to_insert),
            Focus::Password => self.password.enter_char(to_insert),
            Focus::Programms => {}
        }
    }

    pub fn delete_char(&mut self) {
        match self.focus {
            Focus::Username => self.username.delete_char(),
            Focus::Password => self.password.delete_char(),
            Focus::Programms => {}
        }
    }

    pub fn move_cursor_left(&mut self) {
        match self.focus {
            Focus::Username => self.username.move_cursor_left(),
            Focus::Password => self.password.move_cursor_left(),
            Focus::Programms => {}
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.focus {
            Focus::Username => self.username.move_cursor_right(),
            Focus::Password => self.password.move_cursor_right(),
            Focus::Programms => {}
        }
    }

    pub fn cursor_position(&self) -> usize {
        match self.focus {
            Focus::Username => self.username.cursor_position,
            Focus::Password => self.password.cursor_position,
            _ => 0,
        }
    }

    pub fn reset(&mut self) {
        match self.focus {
            Focus::Username => self.username.reset(),
            Focus::Password => self.password.reset(),
            _ => {}
        }
    }
}
