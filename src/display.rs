
use crate::dialog;
use crate::content::{ Content, ContentType };

const BOTTOM_BORDER_SIZE: i32 = 3;

pub struct Screen {

    left_content: Content,
    right_content: Content,
}

impl Screen {

    pub fn new() -> Screen {

        dialog::initialize();

        let left_content = Content::new(true, ContentType::Datasets);
        let right_content = Content::new(false, ContentType::Snapshots);

        Screen { left_content, right_content }
    }

    pub fn run(&mut self) {

        self.update_content();

        loop {

            self.draw();
            if !self.handle_keys() { break; };
            self.update_content();
        }
    }

    fn handle_keys(&mut self) -> bool {

        const KEY_LEFT:  i32 = 0x104;
        const KEY_RIGHT: i32 = 0x105;
        const KEY_UP:    i32 = 0x103;
        const KEY_DOWN:  i32 = 0x102;
        const KEY_PGDN:  i32 = 0x152;
        const KEY_PGUP:  i32 = 0x153;
        const KEY_HOME:  i32 = 0x106;
        const KEY_END:   i32 = 0x168;
        const KEY_TAB:   i32 = 0x009;

        const KEY_F1:    i32 = 0x109;
        const KEY_F2:    i32 = 0x10a;
        const KEY_F3:    i32 = 0x10b;
        const KEY_F4:    i32 = 0x10c;
        const KEY_F5:    i32 = 0x10d;
        const KEY_F6:    i32 = 0x10e;
        const KEY_F7:    i32 = 0x10f;
        const KEY_F8:    i32 = 0x110;
        const KEY_F9:    i32 = 0x111;
        const KEY_F10:   i32 = 0x112;
        const KEY_F11:   i32 = 0x113;
        const KEY_F12:   i32 = 0x114;

        let selected_content = self.selected_content();
        let key = dialog::get_key();

        match key {

            KEY_LEFT  => { self.switch_window(); },
            KEY_RIGHT => { self.switch_window(); },
            KEY_UP    => { selected_content.jump(-1); },
            KEY_DOWN  => { selected_content.jump(1); },
            KEY_PGUP  => { selected_content.jump(-10); }
            KEY_PGDN  => { selected_content.jump(10); }
            KEY_HOME  => { selected_content.jump_to(0); },
            KEY_END   => { selected_content.jump_to_last(); },
            KEY_TAB   => { selected_content.switch_mode(); },

            KEY_F1    => { selected_content.key_f1(); },
            KEY_F2    => { selected_content.key_f2(); },
            KEY_F3    => { selected_content.key_f3(); },
            KEY_F4    => { selected_content.key_f4(); },
            KEY_F5    => { selected_content.key_f5(); },
            KEY_F6    => { selected_content.key_f6(); },
            KEY_F7    => { selected_content.key_f7(); },
            KEY_F8    => { selected_content.key_f8(); },
            KEY_F9    => { selected_content.key_f9(); },
            KEY_F10   => { return false; },
            KEY_F11   => { },
            KEY_F12   => { },

            _ => {},
        }

        true
    }

    fn selected_content(&mut self) -> &mut Content {
        if self.left_content.is_selected {
            &mut self.left_content

        } else {
            &mut self.right_content
        }
    }

    fn switch_window(&mut self) {

        if self.left_content.is_selected {
            self.left_content.is_selected = false;
            self.right_content.is_selected = true;
        } else {
            self.left_content.is_selected = true;
            self.right_content.is_selected = false;
        }
    }

    fn draw(&mut self) {

        self.draw_menu();
        self.draw_panes();
    }

    fn draw_panes(&mut self) {

        let (max_y, max_x) = dialog::screen_dimensions();

        let left_start_y = 0;
        let left_start_x = 0;
        let left_height = max_y;
        let left_width  = max_x/2;

        let left_title = self.left_content.type_text();
        self.left_content.scroll(left_height-BOTTOM_BORDER_SIZE+1);
        let left_pane = dialog::window(left_height-1, left_width, left_start_y, left_start_x, left_title.as_str());
        self.left_content.write(left_pane, left_height-BOTTOM_BORDER_SIZE, left_width);

        let right_start_x = left_width;
        let right_start_y = 0;
        let right_height = max_y;
        let right_width  = max_x - right_start_x;

        let right_title = self.right_content.type_text();
        self.right_content.scroll(right_height-BOTTOM_BORDER_SIZE+1);
        let right_pane = dialog::window(right_height-1, right_width, right_start_y, right_start_x, right_title.as_str());
        self.right_content.write(right_pane, right_height-BOTTOM_BORDER_SIZE, right_width);

        dialog::refresh_screen();
        dialog::refresh(left_pane);
        dialog::refresh(right_pane);
    }

    fn draw_menu(&mut self) {

        let pools_menu     = " 1 Help  2 _____ 3 _____ 4 _____ 5 _____ 6 _____ 7 Scrub 8 Destr 9 GetAl 10 Exit ".to_string();
        let datasets_menu  = " 1 Help  2 _____ 3 _____ 4 _____ 5 Snaps 6 Renam 7 Creat 8 Destr 9 GetAl 10 Exit ".to_string();
        let volumes_menu   = " 1 Help  2 _____ 3 _____ 4 _____ 5 Snaps 6 Renam 7 Creat 8 Destr 9 GetAl 10 Exit ".to_string();
        let snapshots_menu = " 1 Help  2 Diff  3 Send  4 _____ 5 Clone 6 Renam 7 RollB 8 Destr 9 GetAl 10 Exit ".to_string();

        let mut selected_menu: String;

        let selected_content = self.selected_content();
        match selected_content.c_type {
            ContentType::Pools =>     { selected_menu = pools_menu; },
            ContentType::Datasets =>  { selected_menu = datasets_menu; },
            ContentType::Volumes =>   { selected_menu = volumes_menu; },
            ContentType::Snapshots => { selected_menu = snapshots_menu; },
        };

        let (_max_y, max_x) = dialog::screen_dimensions();
        for _ in selected_menu.len()..max_x as usize {
            selected_menu.push_str(" ");
        }

        dialog::bottom_menu(selected_menu.as_str());
    }

    fn update_content(&mut self) {
        self.left_content.update();
        self.right_content.update();
    }
}

impl Drop for Screen {

    fn drop(&mut self) {
        dialog::finish();
    }
}
