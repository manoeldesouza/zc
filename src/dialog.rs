
use std::char;

use ncurses::*;


const KEY_ESC:   i32 = 0x1b;
const KEY_ENTER: i32 = 0xa;
const KEY_TAB:   i32 = 0x009;

pub fn initialize() -> (i32, i32) {

    initscr();
    cbreak();
    // raw();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    // nodelay(stdscr(), true);

    screen_dimensions()
}

pub fn screen_dimensions() -> (i32, i32) {

    let mut max_y: i32 = 0;
    let mut max_x: i32 = 0;

    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    (max_y, max_x)
}

pub fn get_key() -> i32 {
    getch()
}

pub fn refresh(window: WINDOW) {
    wrefresh(window);
}

pub fn refresh_screen() {
    wrefresh(stdscr());
}

pub fn finish() {
    clear();
    endwin();
}

pub fn scroll(position: i32, mut start_from: i32, height: i32) -> i32 {

    if position < 0 {
        start_from = 0;
    } 
    
    if start_from < 0 {
        start_from = 0;
    }
        
    if position < start_from {
        start_from = position;
    } 
    
    if position >= start_from + height-1 {
        start_from = position - height+2;
    }

    start_from
}

pub fn window(height: i32, width: i32, start_y: i32, start_x: i32, title: &str) -> WINDOW {

    let win = newwin(height, width, start_y, start_x);

    box_(win, 0, 0);
    wmove(win, 0, 1);
    wprintw(win, title);

    win
}

pub fn input_dialog(title: &str, prompt: &str, info: &str) -> Result<String,()> {

    let dialog_height = 8;
    let dialog_width = 70;

    let (max_y, max_x) = screen_dimensions();

    let start_y = max_y/2 - dialog_height/2;
    let start_x = max_x/2 - dialog_width/2;

    let footnote = "ESC Cancel     ENTER Confirm";

    let dialog = window(dialog_height, dialog_width, start_y, start_x, title);
    mvwprintw(dialog, 2, 3, prompt);
    wattroff(dialog, A_REVERSE());

    mvwprintw(dialog, 5, 3, "----------------------------------------------------------------");
    let foot_x = dialog_width/2 - footnote.len() as i32/2;
    mvwprintw(dialog, 6, foot_x, footnote);

    wattron(dialog, A_REVERSE());
    mvwprintw(dialog, 3, 3, "                                                                ");
    mvwprintw(dialog, 3, 3, info);

    wrefresh(dialog);

    let mut input = String::from(info);

    loop {
        let key = getch();

        match key {
            KEY_ENTER      => { return Ok(input) },
            KEY_ESC        => { return Err(())   },
            0x20..=0x7f    => { input.push(char::from_u32(key as u32).unwrap()); },
            KEY_BACKSPACE  => { input.pop(); }
            _              => {},
        }

        let input_size = input.len();
        let mut input_scr = input.clone();
        for _ in input_size..64 {
            input_scr.push(' ');
        }

        mvwprintw(dialog, 3, 3, input_scr.as_str());
        wrefresh(dialog);
    }
}

pub fn two_input_dialog(title: &str, prompt: &str, info1: &str, info2: &str) -> Result<(String, String),()> {

    let dialog_height = 10;
    let dialog_width = 70;

    let (max_y, max_x) = screen_dimensions();

    let start_y = max_y/2 - dialog_height/2;
    let start_x = max_x/2 - dialog_width/2;

    let footnote = "ESC Cancel     ENTER Confirm";

    let dialog = window(dialog_height, dialog_width, start_y, start_x, title);
    mvwprintw(dialog, 2, 3, prompt);
    wattroff(dialog, A_REVERSE());

    mvwprintw(dialog, 7, 3, "----------------------------------------------------------------");
    let foot_x = dialog_width/2 - footnote.len() as i32/2;
    mvwprintw(dialog, 8, foot_x, footnote);

    wattron(dialog, A_REVERSE());
    mvwprintw(dialog, 3, 3, "                                                                ");
    mvwprintw(dialog, 3, 3, info1);

    wattron(dialog, A_REVERSE());
    mvwprintw(dialog, 5, 3, "                                                                ");
    mvwprintw(dialog, 5, 3, info2);

    wrefresh(dialog);

    let mut input1 = String::from(info1);
    let mut input2 = String::from(info2);

    let mut is_input1_selected = false;
    if info1.is_empty() {
        is_input1_selected = true;
    };

    loop {
        let key = getch();

        match key {
            KEY_ENTER      => { return Ok((input1, input2)) },
            KEY_ESC        => { return Err(())   },
            0x20..=0x7f    => { 
                if is_input1_selected {
                    input1.push(char::from_u32(key as u32).unwrap());

                } else {
                    input2.push(char::from_u32(key as u32).unwrap());
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
                if is_input1_selected == true {
                    is_input1_selected = false; 
                } else {
                    is_input1_selected = true;
                }
            }
            _              => {},
        }

        let input_size1 = input1.len();
        let mut input_scr1 = input1.clone();
        for _ in input_size1..64 {
            input_scr1.push(' ');
        }

        let input_size2 = input2.len();
        let mut input_scr2 = input2.clone();
        for _ in input_size2..64 {
            input_scr2.push(' ');
        }

        mvwprintw(dialog, 3, 3, input_scr1.as_str());
        mvwprintw(dialog, 5, 3, input_scr2.as_str());
        wrefresh(dialog);
    }
}

pub fn confirm_dialog(title: &str, prompt: &str, info: &str) -> Result<(),()> {

    let dialog_height = 8;
    let dialog_width = 70;

    let (max_y, max_x) = screen_dimensions();

    let start_y = max_y/2 - dialog_height/2;
    let start_x = max_x/2 - dialog_width/2;

    let footnote = "ESC Cancel     ENTER Confirm";

    let dialog = window(dialog_height, dialog_width, start_y, start_x, title);
    mvwprintw(dialog, 2, 3, prompt);
    mvwprintw(dialog, 3, 3, info);

    mvwprintw(dialog, 5, 3, "----------------------------------------------------------------");
    let foot_x = dialog_width/2 - footnote.len() as i32/2;
    mvwprintw(dialog, 6, foot_x, footnote);

    wrefresh(dialog);

    loop {
        let key = getch();

        if key == KEY_ENTER {
            return Ok(())
        }

        if key == KEY_ESC {
            return Err(())
        }
    }
}

pub fn result_dialog(title: &str, prompt: &str, info: Vec<&str>) {

    let (max_y, max_x) = screen_dimensions();

    let dialog_height = max_y - 6;
    let dialog_width = max_x - 6;

    let start_y = max_y/2 - dialog_height/2;
    let start_x = max_x/2 - dialog_width/2;

    let footnote = "ENTER Close";
    let bar = "----------------------------------------------------------------";

    let dialog = window(dialog_height, dialog_width, start_y, start_x, title);
    mvwprintw(dialog, 2, 3, prompt);

    let bar_x = dialog_width/2 - bar.len() as i32/2;
    mvwprintw(dialog, dialog_height-3, bar_x, bar);
    let foot_x = dialog_width/2 - footnote.len() as i32/2;
    mvwprintw(dialog, dialog_height-2, foot_x, footnote);

    let mut start_from = 0;
    let height = dialog_height - 6;
    let width = (dialog_width - 6) as usize;

    loop {

        if start_from < 0 {
            start_from = 0;
        }

        if start_from >= info.len() as i32 - height {
            start_from = info.len() as i32 - height;
        }

        for (i, line) in info.iter().enumerate() {

            if (i as i32) < start_from { continue }
            if (i as i32) >= height + start_from { break }

            let text = fit_to_window(line, width);

            mvwprintw(dialog, 3+(i as i32)-start_from, 3, text.as_str());
        }   

        wrefresh(dialog);

        let key = getch();

        if key == KEY_ENTER || key == KEY_ESC || key == KEY_F10 {
            return
        } else if key == KEY_UP {
            start_from -= 1;
        } else if key == KEY_DOWN {
            start_from += 1;
        } else if key == KEY_PPAGE {
            start_from -= 10;
        } else if key == KEY_NPAGE {
            start_from += 10;
        }
    }
}

pub fn bottom_menu(text: &str) {

    let (max_y, _max_x) = screen_dimensions();

    wattron(stdscr(), A_BOLD());
    mvwprintw(stdscr(), max_y-1, 0, text);
    wattroff(stdscr(), A_BOLD());
}

pub fn fit_to_window(result_name: &str, width: usize) -> String {

    let mut name = result_name.to_string();

    if name.len() > width-2 {
        name = name.get(0..width-2).unwrap().to_string();
    }

    for _ in name.len()..width-2 {
        name.push_str(" ");
    }

    format!("{}", name)
}

// pub fn print_in_position(text: &str, y: i32, x: i32) {
//     mvprintw(y, x, text);
// }
