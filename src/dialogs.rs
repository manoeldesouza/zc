
use ncurses::*;
use crate::contents::{ Content, ContentType };

const MIN_HEIGHT: i32 = 7;
const DEFAULT_WIDTH: i32 = 80;

const BAR: &str = "-------------------------------------";
const SPACE: &str = "                                                                ";

const KEY_TAB:   i32 = 0x009;
const KEY_ENTER: i32 = 0x00a;
const KEY_ESC:   i32 = 0x1b;

pub fn initialize() {

    initscr();
    cbreak();
    // raw();

    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    // nodelay(stdscr(), true);
}

pub fn finish() {
    clear();
    endwin();
}

pub fn screen_dimensions() -> (i32, i32) {

    let mut max_y: i32 = 0;
    let mut max_x: i32 = 0;

    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    (max_y, max_x)
}

pub fn dual_pane(left_content: &mut Content, right_content: &mut Content) {

    let (height, max_x) = screen_dimensions();
    let mut width;

    width = max_x / 2;
    let left_pane = window(height - 1, width, 0, 0, &left_content.title());
    left_content.scroll(height - 2);
    write_at(left_pane, left_content, height - 3, width);

    width = max_x - width;
    let right_pane = window(height - 1, width, 0, width - 1, &right_content.title());
    right_content.scroll(height - 2);
    write_at(right_pane, right_content, height - 3, width);

    refresh();
    wrefresh(left_pane);
    wrefresh(right_pane);

    delwin(left_pane);
    delwin(right_pane);
}

pub fn handle_keys(left: &mut Content, right: &mut Content) -> bool {

    const KEY_TAB: i32 = 0x009;

    let (selected_content, other_content) = {
        if   left.is_selected { (left, right) }
        else                  { (right, left) }
    };

    bottom_menu(&selected_content.content_type);

    let key = getch();

    match key {

        KEY_LEFT  => { switch_window(selected_content, other_content); },
        KEY_RIGHT => { switch_window(selected_content, other_content); },
        KEY_UP    => { selected_content.jump(-1); },
        KEY_DOWN  => { selected_content.jump(1); },
        KEY_PPAGE => { selected_content.jump(-10); }
        KEY_NPAGE => { selected_content.jump(10); }
        KEY_HOME  => { selected_content.jump_to(0); },
        KEY_END   => { selected_content.jump_to_last(); },
        KEY_TAB   => { selected_content.next(); },

        KEY_F1    => { selected_content.key_f(1); },
        KEY_F2    => { selected_content.key_f(2); },
        KEY_F3    => { selected_content.key_f(3); },
        KEY_F4    => { selected_content.key_f(4); },
        KEY_F5    => { selected_content.key_f(5); },
        KEY_F6    => { selected_content.key_f(6); },
        KEY_F7    => { selected_content.key_f(7); },
        KEY_F8    => { selected_content.key_f(8); },
        KEY_F9    => { selected_content.key_f(9); },
        KEY_F10   => { return true; },
        KEY_F11   => { selected_content.key_f(11); },
        KEY_F12   => { selected_content.key_f(12); },

        _ => { },
    }

    false
}

pub fn presentation_box(title: &str, prompt: &str, message: Vec<String>) {

    let (max_y, max_x) = screen_dimensions();
    let (height, width) = (max_y - 6, max_x - 8);
    let (start_y, start_x) = center_window(height, width);
    let footnote = "Press F10 to close";
    let dialog = window(height, width, start_y, start_x, title);

    mvwprintw(dialog, 2, 3, prompt);
    write_footnote(dialog, height, width, footnote);

    let mut start_from = 0;
    loop {

        if start_from < 0 || message.len() < height as usize {
            start_from = 0;

        } else if start_from >= message.len() as i32 - (height - 6) {
            start_from = message.len() as i32 - (height - 6);
        }

        for (i, line) in message.iter().enumerate() {

            if (i as i32) < start_from { continue }
            if (i as i32) >= height + start_from - 6 { break }

            let text = fit_to_window(line, width as usize - 6);
            mvwprintw(dialog, 3+(i as i32)-start_from, 3, &text);
        }

        wrefresh(dialog);

        let key = getch();
        match key {
            KEY_ENTER | KEY_ESC | KEY_F10 => { delwin(dialog); return; },
            KEY_UP    => { start_from -= 1; },
            KEY_DOWN  => { start_from += 1; },
            KEY_PPAGE => { start_from -= 10; },
            KEY_NPAGE => { start_from += 10; },
            _ => { },
        }
    } 
}

pub fn navigation_box(title: &str, prompt: &str, message: Vec<String>) -> Result<String,()> {

    let (max_y, max_x) = screen_dimensions();
    let (height, width) = (max_y - 6, max_x - 8);
    let (start_y, start_x) = center_window(height, width);
    let footnote = "ENTER Modify  F2 New  F10 close";
    let dialog = window(height, width, start_y, start_x, title);

    mvwprintw(dialog, 2, 3, prompt);
    write_footnote(dialog, height, width, footnote);

    let mut start_from = 0;
    let mut position = 0;

    loop {

        if position >= message.len() as i32 - 1 { 
            position = message.len() as i32 - 1;
        }

        if position < 0 {
            position = 0;
        }

        if position < start_from {
            start_from = position;
        }

        if position > start_from + (height - 7) {
            start_from = position - (height - 7);
        }

        if start_from < 0 || message.len() + 6 < height as usize {
            start_from = 0;
        }

        for (i, line) in message.iter().enumerate() {

            if (i as i32) < start_from { continue }
            if (i as i32) >= height + start_from - 6 { break }
            if (i as i32) == position { wattron(dialog, A_REVERSE()); }

            let text = fit_to_window(line, width as usize - 6);
            mvwprintw(dialog, 3+(i as i32)-start_from, 3, &text);
            wattroff(dialog, A_REVERSE());

        }

        wrefresh(dialog);

        let key = getch();
        match key {
            KEY_ENTER => { 
                delwin(dialog); 
                return Ok(message.get(position as usize).unwrap().to_string());
            },
            KEY_F2    => { 
                delwin(dialog); 
                return Ok(String::new());
            },
            KEY_ESC | KEY_F10 => { delwin(dialog); return Err(()); },
            KEY_UP    => { position -= 1; },
            KEY_DOWN  => { position += 1; },
            KEY_PPAGE => { position -= 10; },
            KEY_NPAGE => { position += 10; },
            _ => { },
        }
    } 
}

pub fn confirmation_box(title: &str, prompt: &str, message: Vec<String>) -> bool {

    let (height, width) = (MIN_HEIGHT + message.len() as i32, DEFAULT_WIDTH);
    let (start_y, start_x) = center_window(height, width);
    let footnote = "ENTER Confirm   Other key to cancel";
    let dialog = window(height, width, start_y, start_x, title);

    mvwprintw(dialog, 2, 3, prompt);
    for (i, value) in message.iter().enumerate() {
        mvwprintw(dialog, 3 + i as i32, 3, value);
    }
    write_footnote(dialog, height, width, footnote);

    refresh();
    wrefresh(dialog);

    let key = getch();
    delwin(dialog);

    if key == KEY_ENTER { true  }
    else                { false }
}

pub fn message_box(title: &str, prompt: &str, message: Vec<String>) {

    let (height, width) = (MIN_HEIGHT + message.len() as i32, DEFAULT_WIDTH);
    let (start_y, start_x) = center_window(height, width);
    let footnote = "Press any key to close";
    let dialog = window(height, width, start_y, start_x, title);

    mvwprintw(dialog, 2, 3, prompt);
    for (i, value) in message.iter().enumerate() {
        mvwprintw(dialog, 3 + i as i32, 3, value);
    }
    write_footnote(dialog, height, width, footnote);

    refresh();
    wrefresh(dialog);

    getch();
    delwin(dialog);
}

pub fn single_input_box(title: &str, prompt: &str, default_value: String) -> String {

    let (height, width) = (MIN_HEIGHT + 1, DEFAULT_WIDTH);
    let (start_y, start_x) = center_window(height, width);
    let footnote = "ENTER Confirm   F10 cancel";
    let dialog = window(height, width, start_y, start_x, title);

    mvwprintw(dialog, 2, 3, prompt);
    write_footnote(dialog, height, width, footnote);
    let mut input = format!("{}", default_value);

    loop {

        let input_str = format!("{} ", input);
        wattron(dialog, A_REVERSE());
        mvwprintw(dialog, 3, 3, SPACE);
        mvwprintw(dialog, 3, 3, &input_str);
        wattroff(dialog, A_REVERSE());

        refresh();
        wrefresh(dialog);

        let key = getch();
        match key {
            KEY_ENTER      => { delwin(dialog); return input; },
            KEY_ESC        => { delwin(dialog); return format!(""); },
            KEY_F10        => { delwin(dialog); return format!(""); },
            0x20..=0x7f    => { input.push(std::char::from_u32(key as u32).unwrap()); },
            KEY_BACKSPACE  => { input.pop(); }
            _              => {},
        }        
    }
}

pub fn dual_input_box(title: &str, prompt: &str, default_value1: String, default_value2: String) -> (String, String) {

    let (height, width) = (MIN_HEIGHT + 3, DEFAULT_WIDTH);
    let (start_y, start_x) = center_window(height, width);
    let footnote = "ENTER Confirm   F10 cancel";
    let dialog = window(height, width, start_y, start_x, title);

    mvwprintw(dialog, 2, 3, prompt);
    write_footnote(dialog, height, width, footnote);
    let mut input1 = format!("{}", default_value1);
    let mut input2 = format!("{}", default_value2);
    let mut is_input1_selected = true;

    loop {

        let input_str1 = format!("{} ", input1);
        let input_str2 = format!("{} ", input2);
        wattron(dialog, A_REVERSE());
        mvwprintw(dialog, 3, 3, SPACE);
        mvwprintw(dialog, 3, 3, &input_str1);
        mvwprintw(dialog, 5, 3, SPACE);
        mvwprintw(dialog, 5, 3, &input_str2);
        wattroff(dialog, A_REVERSE());

        refresh();
        wrefresh(dialog);

        let key = getch();
        match key {
            KEY_ENTER      => { 
                delwin(dialog); 
                return (input1, input2);
            },
            KEY_ESC | KEY_F10 => { 
                delwin(dialog);
                return (String::new(), String::new());
            },
            0x20..=0x7f    => {
                if is_input1_selected {
                    input1.push(std::char::from_u32(key as u32).unwrap());
                } else {
                    input2.push(std::char::from_u32(key as u32).unwrap());
                }
            },
            KEY_BACKSPACE  => { 
                if is_input1_selected {
                    input1.pop();
                } else {
                    input2.pop();
                }
            },
            KEY_TAB        => {
                is_input1_selected = !is_input1_selected;
            },
            _              => {},
        }        
    }
}

fn bottom_menu(content_type: &ContentType) {

    let pools     = String::from(" F1 Help  F2 ____  F3 ____  F4 ____  F5 ____  F6 ____  F7 Scrub  F8 Destroy  F9 Get all  F10 Exit");
    let volumes   = String::from(" F1 Help  F2 ____  F3 ____  F4 ____  F5 Snapshot  F6 Rename  F7 Create  F8 Destroy  F9 Get all  F10 Exit");
    let datasets  = String::from(" F1 Help  F2 Promote  F3 ____  F4 ____  F5 Snapshot  F6 Rename  F7 Create  F8 Destroy  F9 Get all  F10 Exit");
    let snapshots = String::from(" F1 Help  F2 Diff  F3 Send  F4 ____  F5 Clone  F6 Rename  F7 Rollback  F8 Destroy  F9 Get all  F10 Exit");

    let mut menu = match content_type {
        ContentType::Pools =>     { pools     },
        ContentType::Datasets =>  { datasets  },
        ContentType::Volumes =>   { volumes   },
        ContentType::Snapshots => { snapshots },
    };

    let (height, width) = screen_dimensions();

    for _ in menu.len()..(width as usize) {
        menu.push(' ');
    }

    mvprintw(height - 1, 0, &menu);
}

fn write_at(pane: WINDOW, content: &Content, height: i32, width: i32) {

    for (i, line) in content.list.iter().enumerate() {

        if (i as i32) <  content.start { continue }
        if (i as i32) >= height + content.start { break }
        if (i as i32) == content.position && content.is_selected { wattron(pane, A_REVERSE()); }

        let text = fit_to_window(line.as_str(), width as usize);
        let content_position = i as i32 - content.start + 1;

        mvwprintw(pane, content_position, 1, text.as_str());
        wattroff(pane, A_REVERSE());
    }  
}

fn write_footnote(dialog: WINDOW, height: i32, width: i32, footnote: &str) {

    let start_y = height - 3;
    let start_x = width / 2 - BAR.len() as i32 / 2;
    mvwprintw(dialog, start_y, start_x, BAR);

    let start_y = height - 2;
    let start_x = width / 2 - footnote.len() as i32 / 2;
    mvwprintw(dialog, start_y, start_x, footnote);
}

fn fit_to_window(source_line: &str, width: usize) -> String {

    let mut line = source_line.to_string();

    if line.len() > width-2 {
        line = line.get(0..width-2).unwrap().to_string();
    }   

    for _ in line.len()..width-2 {
        line.push_str(" ");
    }   

    line
}

fn window(height: i32, width: i32, start_y: i32, start_x: i32, title: &str) -> WINDOW {

    let win = newwin(height, width, start_y, start_x);
    let padded_title = format!(" {} ", title);

    box_(win, 0, 0); 

    wattron(win, A_BOLD());
    mvwprintw(win, 0, 2, &padded_title);
    wattroff(win, A_BOLD());

    win 
}

fn switch_window(left: &mut Content, right: &mut Content) {
    left.is_selected = false;
    right.is_selected = true;
}

fn center_window(height: i32, width: i32) -> (i32, i32) {

    let (max_y, max_x) = screen_dimensions();

    let start_y = max_y / 2 - height / 2;
    let start_x = max_x / 2 - width / 2;

    (start_y, start_x)
}