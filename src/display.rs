
use std::thread;
use std::time;

extern crate ncurses;
use ncurses::*;

use crate::command;

const BOTTOM_BORDER_SIZE: usize = 3;


pub struct Screen {

    max_y: i32,
    max_x: i32,

    left_content: Content,
    right_content: Content,
}

impl Screen {

    pub fn new() -> Screen {

        initscr();
        cbreak();
        // raw();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        keypad(stdscr(), true);
        // nodelay(stdscr(), true);

        let mut max_y: i32 = 0;
        let mut max_x: i32 = 0;
        
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        let left_content = Content {
            is_selected: true,
            start_from: 0,
            position: 0,
            c_type: ContentType::Datasets,
            command_result: Vec::new(),
            selected_elements: Vec::new(),
        };

        let right_content = Content {
            is_selected: false,
            start_from: 0,
            position: 0,
            c_type: ContentType::Snapshots,
            command_result: Vec::new(),
            selected_elements: Vec::new(),
        };

        Screen {
            max_y: 0,
            max_x: 0,

            left_content,
            right_content,
        }
    }

    pub fn run(&mut self) {

        loop {
            self.draw();

            if self.handle_keys() == Err(()) { break; };

            thread::sleep(time::Duration::from_millis(10));
        }
    }

    fn handle_keys(&mut self) -> Result<(),()> {

        const KEY_TAB: i32 = 0x9;
        const KEY_PUP: i32 = 0x153;
        const KEY_PDN: i32 = 0x152;

        let key = wgetch(stdscr());

        match key {
            KEY_F1    => {},
            KEY_F2    => {},
            KEY_F3    => {},
            KEY_F4    => {},
            KEY_F5    => {},
            KEY_F6    => {},
            KEY_F7    => {},
            KEY_F8    => { self.key_f8(); },
            KEY_F9    => {},
            KEY_F10   => { return Err(()); },
            KEY_F11   => {},
            KEY_F12   => { self.test_windows(); },

            KEY_DL    => {},

            KEY_HOME  => { self.key_home(); },
            KEY_END   => { self.key_end(); },

            KEY_UP    => { self.key_up(); },
            KEY_DOWN  => { self.key_down(); },

            KEY_PUP   => { self.key_pgup(); }
            KEY_PDN   => { self.key_pgdown(); }

            KEY_LEFT  => { self.switch_window(); },
            KEY_RIGHT => { self.switch_window(); },

            KEY_IL    => {},
            KEY_TAB   => { self.switch_window(); },

            _ => {},
        }

        Ok(())
    }

    fn test_windows(&self) {

        let s = format!("  Left position: {} len: {}  ", self.left_content.position, self.left_content.command_result.len());
        mvwprintw(stdscr(), 1, 1, s.as_str());
        let s = format!(" Right position: {} len: {}  ", self.right_content.position, self.right_content.command_result.len());
        mvwprintw(stdscr(), 2, 1, s.as_str());
        getch();
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

    fn key_home(&mut self) {

        if self.left_content.is_selected {
            self.left_content.position = 0;
        } else {
            self.right_content.position = 0;
        }
    }

    fn key_end(&mut self) {

        if self.left_content.is_selected {
            self.left_content.position = self.left_content.command_result.len()-1;
        } else {
            self.right_content.position = self.right_content.command_result.len()-1;
        }
    }

    fn key_up(&mut self) {

        if self.left_content.is_selected && self.left_content.position > 0 {
            self.left_content.position -= 1;
        } else if self.right_content.is_selected && self.right_content.position > 0{
            self.right_content.position -= 1;
        }
    }

    fn key_down(&mut self) {

        if self.left_content.is_selected && self.left_content.position+2 < self.left_content.command_result.len() {
            self.left_content.position += 1;
    
        } else if self.right_content.is_selected && self.right_content.position+2 < self.right_content.command_result.len() {
            self.right_content.position += 1;                
        }
    }

    fn key_pgup(&mut self) {

        if self.left_content.is_selected {
            if self.left_content.position > 10 {
                self.left_content.position -= 10;
            } else {
                self.left_content.position = 0;
            }
        } else if self.right_content.is_selected {
            if self.right_content.position > 10 {
                self.right_content.position -= 10;
            } else {
                self.right_content.position = 0;
            }
        }
    }

    fn key_pgdown(&mut self) {

        if self.left_content.is_selected {
            if self.left_content.position+11 < self.left_content.command_result.len() {
                self.left_content.position += 10;
            } else {
                self.left_content.position = self.left_content.command_result.len()-2;
            }
        } else if self.right_content.is_selected {
            if self.right_content.position+11 < self.right_content.command_result.len() {
                self.right_content.position += 10;
            } else {
                self.right_content.position = self.right_content.command_result.len()-2;
            }
        }
    }

    fn scroll_window(content: &mut Content, height: i32) {

        if content.position < content.start_from {
            content.start_from = content.position

        } else if content.position - content.start_from > (height as usize - BOTTOM_BORDER_SIZE - 1) {
            content.start_from = content.position - (height as usize - BOTTOM_BORDER_SIZE - 1);
        }
    }

    fn key_f8(&self) {

        let selected_elements = self.selected_elements();

        match self.content_type() {
            ContentType::Pools =>     {  },
            ContentType::Datasets =>  { command::zfs_destroy(selected_elements) },
            ContentType::Volumes =>   {  },
            ContentType::Snapshots => { command::zfs_destroy(selected_elements) },
        };
    }

    fn selected_elements(&self) -> Vec<String> {

        if self.left_content.is_selected {
            if self.left_content.selected_elements.len() > 0 {
                self.left_content.selected_elements.to_owned()
            } else {
                vec![self.left_content.command_result[self.left_content.position].name.to_owned()]
            }

        } else {
            if self.right_content.selected_elements.len() > 0 {
                self.right_content.selected_elements.to_owned()
            } else {
                vec![self.right_content.command_result[self.right_content.position].name.to_owned()]
            }
        }
    }

    fn draw(&mut self) {

        getmaxyx(stdscr(), &mut self.max_y, &mut self.max_x);

        self.draw_menu();
        self.draw_content();
    }

    fn draw_content(&mut self) {

        let left_start_y = 0;
        let left_start_x = 0;
        let left_height = self.max_y;
        let left_width  = self.max_x/2;
        let left_title = self.left_content.c_type.text();

        let left_window = Screen::draw_window(left_height-1, left_width, left_start_y, left_start_x, left_title.as_str());
        self.left_content.update();
        Screen::scroll_window(&mut self.left_content, left_height);
        Screen::write_content(&self.left_content, left_window, left_height, left_width);

        let right_start_x = left_width;
        let right_start_y = 0;
        let right_height = self.max_y;
        let right_width  = self.max_x - right_start_x;
        let right_title = self.right_content.c_type.text();

        let right_window = Screen::draw_window(right_height-1, right_width, right_start_y, right_start_x, right_title.as_str());
        self.right_content.update();
        Screen::scroll_window(&mut self.right_content, right_height);
        Screen::write_content(&self.right_content, right_window, right_height, right_width);

        wrefresh(stdscr());
        wrefresh(left_window);
        wrefresh(right_window);
    }

    fn draw_window(height: i32, width: i32, start_y: i32, start_x: i32, title: &str) -> WINDOW {

        let win = newwin(height, width, start_y, start_x);

        box_(win, 0, 0);
        wmove(win, 0, 1);
        wprintw(win, title);

        win
    }

    fn draw_menu(&mut self) {

        let pools_menu     = format!("P: 1_____ 2_____ 3_____ 4_____ 5_____ 6_____ 7_____ 8_____ 9_____ 10Exit ");
        let datasets_menu  = format!("D: 1_____ 2_____ 3_____ 4_____ 5_____ 6_____ 7_____ 8Remov 9_____ 10Exit ");
        let volumes_menu   = format!("V: 1_____ 2_____ 3_____ 4_____ 5_____ 6_____ 7_____ 8_____ 9_____ 10Exit ");
        let snapshots_menu = format!("S: 1_____ 2_____ 3_____ 4_____ 5_____ 6_____ 7_____ 8Remov 9_____ 10Exit ");

        let mut selected_menu: String;

        match self.content_type() {
            ContentType::Pools =>     { selected_menu = pools_menu; },
            ContentType::Datasets =>  { selected_menu = datasets_menu; },
            ContentType::Volumes =>   { selected_menu = volumes_menu; },
            ContentType::Snapshots => { selected_menu = snapshots_menu; },
        };

        for _ in selected_menu.len()..self.max_x as usize {
            selected_menu.push_str(" ");
        }

        wattron(stdscr(), A_BOLD());
        mvwprintw(stdscr(), self.max_y-1, 0, selected_menu.as_str());
        wattroff(stdscr(), A_BOLD());
    }

    fn content_type(&self) -> &ContentType {

        if self.left_content.is_selected {
            &self.left_content.c_type
        } else {
            &self.right_content.c_type 
        }
    }

    fn write_content(content: &Content, window: WINDOW, height: i32, width: i32) {

        const TOP_CONTENT_Y: i32 = 1;
        const TOP_CONTENT_X: i32 = 1;

        for (i, result_line) in content.command_result.iter().enumerate() {

            if i < content.start_from { continue }
            if i >= height as usize + content.start_from - BOTTOM_BORDER_SIZE { break }
            if i == content.position && content.is_selected { wattron(window, A_REVERSE()); }

            let text = Screen::fit_to_window(result_line.name.as_str(), width as usize);

            let content_position = i as i32 - content.start_from as i32 + TOP_CONTENT_Y;

            mvwprintw(window, content_position, TOP_CONTENT_X, text.as_str());
            wattroff(window, A_REVERSE());
        }
    }

    fn fit_to_window(result_name: &str, width: usize) -> String {

        let mut name = result_name.to_string();

        if name.len() > width-2 {
            name = name.get(0..width-2).unwrap().to_string();
        }

        for _ in name.len()..width-2 {
            name.push_str(" ");
        }

        format!("{}", name)
    }
}

impl Drop for Screen {

    fn drop(&mut self) {
        clear();
        endwin();
    }
}

#[allow(dead_code)]
enum ContentType {
    Datasets,
    Pools,
    Volumes,
    Snapshots,
}

impl ContentType {

    pub fn text(&self) -> String {
        match self {
            ContentType::Pools => " Pools: ".to_string(),
            ContentType::Datasets => " Datasets: ".to_string(),
            ContentType::Volumes => " Volumes: ".to_string(),
            ContentType::Snapshots => " Snapshots: ".to_string(),
        }
    }
}

struct Content {

    is_selected: bool,
    start_from: usize,
    position: usize,
    c_type: ContentType,
    command_result: Vec<command::CommandResult>,
    selected_elements: Vec<String>,
}

impl Content {

    pub fn update(&mut self) {

        match self.c_type {
            ContentType::Pools     => { self.command_result = command::zfs_pools(); },
            ContentType::Datasets  => { self.command_result = command::zfs_dataset(); },
            ContentType::Volumes   => { self.command_result = command::zfs_volumes(); },
            ContentType::Snapshots => { self.command_result = command::zfs_snapshots(); },
        }
    } 
}