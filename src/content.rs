
use ncurses::*;

use crate::command;
use crate::dialog;

pub enum ContentType {
    Datasets,
    Pools,
    Volumes,
    Snapshots,
}

pub struct Content {

    pub is_selected: bool,
    pub start_from: i32,
    pub position: i32,
    pub c_type: ContentType,
    pub command_result: Vec<command::CommandResult>,
    pub selected_elements: Vec<String>,
}

impl Content {

    pub fn new(is_selected: bool, c_type: ContentType) -> Content {

        Content {
            is_selected: is_selected,
            start_from: 0,
            position: 0,
            c_type: c_type,
            command_result: Vec::new(),
            selected_elements: Vec::new(),
        }
    }

    pub fn update(&mut self) {

        match self.c_type {
            ContentType::Pools     => { self.command_result = command::list_pools(); },
            ContentType::Datasets  => { self.command_result = command::list_dataset(); },
            ContentType::Volumes   => { self.command_result = command::list_volumes(); },
            ContentType::Snapshots => { self.command_result = command::list_snapshots(); },
        }
    } 

    pub fn next(&mut self) -> ContentType {

        match self.c_type {
            ContentType::Pools     => { ContentType::Datasets },
            ContentType::Datasets  => { ContentType::Volumes },
            ContentType::Volumes   => { ContentType::Snapshots },
            ContentType::Snapshots => { ContentType::Pools },
        }

    }

    pub fn jump_to(&mut self, position: i32) {

        if position < 0 || self.command_result.len() == 0 {
            self.position = 0   

        } else if position as usize >= self.command_result.len()-2 {
            self.jump_to_last();

        } else {
            self.position = position;
        }       
    }

    pub fn jump_to_last(&mut self) {
        self.position = (self.command_result.len() as i32) - 2;
    }

    pub fn jump(&mut self, elements: i32) {

        let position = self.position + elements;
        self.jump_to(position);
    }

    pub fn scroll(&mut self, height: i32) {
        self.start_from = dialog::scroll(self.position, self.start_from, height);
    }

    pub fn write(&self, window: WINDOW, height: i32, width: i32) {

        const TOP_CONTENT_Y: i32 = 1;
        const TOP_CONTENT_X: i32 = 1;
    
        for (i, result_line) in self.command_result.iter().enumerate() {

            if (i as i32) < self.start_from { continue }
            if (i as i32) >= height + self.start_from { break }
            if (i as i32) == self.position  && self.is_selected { wattron(window, A_REVERSE()); }

            let text = dialog::fit_to_window(result_line.name.as_str(), width as usize);

            let content_position = i as i32 - self.start_from + TOP_CONTENT_Y;

            mvwprintw(window, content_position, TOP_CONTENT_X, text.as_str());
            wattroff(window, A_REVERSE());
        }
    }

    pub fn switch_mode(&mut self) {
        *self = Content::new(true, self.next());
    }

    pub fn type_text(&self) -> String {
        match self.c_type {
            ContentType::Pools => " Pools: ".to_string(),
            ContentType::Datasets => " Datasets: ".to_string(),
            ContentType::Volumes => " Volumes: ".to_string(),
            ContentType::Snapshots => " Snapshots: ".to_string(),
        }
    }

    pub fn key_f1(&mut self) {
        // TODO
    }

    pub fn key_f2(&mut self) {

        let selected_elements = self.selected_elements();

        match self.c_type {
            ContentType::Pools =>     { },
            ContentType::Datasets =>  { },
            ContentType::Volumes =>   { },
            ContentType::Snapshots => { Content::input_snapshot_diff(selected_elements); },
        }
    }

    pub fn key_f3(&mut self) {
        // TODO
    }

    pub fn key_f4(&mut self) {
        // TODO
    }

    pub fn key_f5(&self) { 

        let selected_elements = self.selected_elements();

        match self.c_type {
            ContentType::Pools =>     { },
            ContentType::Datasets =>  { Content::input_dataset_snapshot(selected_elements); },
            ContentType::Volumes =>   { Content::input_dataset_snapshot(selected_elements); },
            ContentType::Snapshots => { Content::input_snapshot_clone(selected_elements);   },
        };
    }

    pub fn key_f6(&self) { 

        let selected_elements = self.selected_elements();

        match self.c_type {
            ContentType::Pools =>     { },
            ContentType::Datasets =>  { Content::input_dataset_rename(selected_elements); },
            ContentType::Volumes =>   { Content::input_dataset_rename(selected_elements); },
            ContentType::Snapshots => { Content::input_dataset_rename(selected_elements); },
        };
    }

    pub fn key_f7(&self) {

        let selected_elements = self.selected_elements();

        match self.c_type {
            ContentType::Pools =>     { Content::confirm_pool_scrub(selected_elements); },
            ContentType::Datasets =>  { Content::input_dataset_create(selected_elements); },
            ContentType::Volumes =>   { Content::input_volume_create(); },
            ContentType::Snapshots => { Content::confirm_snapshot_rollback(selected_elements); },
        }
    }

    pub fn key_f8(&self) {

        let selected_elements = self.selected_elements();
        let selected_string = Content::seleted_string(&selected_elements);

        let title = " Confirm Destroy: ";
        let prompt = "The following element(s) will be destroyed: ";

        if let Err(_) = dialog::confirm_dialog(title, prompt, selected_string.as_str()) {
            return;
        }

        match self.c_type {
            ContentType::Pools =>     { command::zpool_destroy(selected_elements) },
            ContentType::Datasets =>  { command::zfs_destroy(selected_elements) },
            ContentType::Volumes =>   { command::zfs_destroy(selected_elements) },
            ContentType::Snapshots => { command::zfs_destroy(selected_elements) },
        }
    }

    pub fn key_f9(&self) {
        // TODO
    }

    pub fn selected_elements(&self) -> Vec<String> {

        if self.selected_elements.len() > 0 {
            self.selected_elements.to_owned()
        } else {
            vec![self.command_result[self.position as usize].name.to_owned()]
        }
    }

    fn seleted_string(selected_elements: &Vec<String>) -> String {

        if selected_elements.len() == 1 {
            selected_elements.get(0).unwrap().to_string()
        } else {
            format!("{} elements", selected_elements.len())
        }
    }

    fn confirm_pool_scrub(selected_elements: Vec<String>) {

        let selected_string = Content::seleted_string(&selected_elements);
        let title = " Confirm Scrub: ";
        let prompt = "The following pools(s) will be scrubbed: ";

        if let Err(_) = dialog::confirm_dialog(title, prompt, selected_string.as_str()) {
            return;
        }

        command::zpool_scrub(selected_elements);
    }

    fn confirm_snapshot_rollback(selected_elements: Vec<String>) {

        let selected_string = Content::seleted_string(&selected_elements);
        let title = " Confirm Rollback: ";
        let prompt = "The Dataset(s) will be rolled back to the following snapshot(s): ";

        if let Err(_) = dialog::confirm_dialog(title, prompt, selected_string.as_str()) {
            return;
        }

        command::zfs_rollback(selected_elements);
    }

    fn input_dataset_create(selected_elements: Vec<String>) {

        let selected_string = Content::seleted_string(&selected_elements);

        match dialog::input_dialog(" Create Dataset: ", "Enter the name of the new Dataset", selected_string.as_str()) {
            Ok(dataset_name) => { command::zfs_create(dataset_name); },
            Err(_)    => { },
        }

        dialog::refresh_screen();
    }

    fn input_volume_create() {

        let default_size = "1g";

        match dialog::two_input_dialog(" Create Dataset: ", "Enter the name and size of the new Volume", "", default_size) {
            Ok(volume) => { 

                let volume_name = volume.0;
                let volume_size = volume.1;

                command::volume_create(volume_name, volume_size);
            },
            Err(_)    => { },
        }

        dialog::refresh_screen();
    }

    fn input_dataset_snapshot(selected_elements: Vec<String>) {

        let selected_string = Content::seleted_string(&selected_elements);
        let snapshot = format!("{}@", selected_string);

        match dialog::input_dialog(" Snapshot Dataset: ", "Enter the name of the new Snapshot", snapshot.as_str()) {
            Ok(dataset_name) => { command::zfs_snapshot(dataset_name); },
            Err(_)    => { },
        }

        dialog::refresh_screen();
    }

    fn input_snapshot_diff(selected_elements: Vec<String>) {

        let selected_string = Content::seleted_string(&selected_elements);

        match dialog::two_input_dialog(" Diff Dataset: ", "Enter the name of the first and second Snapshots", selected_string.as_str(), selected_string.as_str()) {
            Ok(snapshots) => { 

                let snapshot_1 = snapshots.0;
                let snapshot_2 = snapshots.1;

                command::zfs_diff(snapshot_1, snapshot_2);
            },
            Err(_)    => { },
        }

        dialog::refresh_screen();
    }

    fn input_snapshot_clone(selected_elements: Vec<String>) {

        let selected_string = Content::seleted_string(&selected_elements);

        match dialog::input_dialog(" Clone Snapshot: ", "Enter the name of the new Snapshot", "") {
            Ok(dataset_name) => { command::zfs_clone(selected_string, dataset_name); },
            Err(_)    => { },
        }

        dialog::refresh_screen();
    }

    fn input_dataset_rename(selected_elements: Vec<String>) {

        let selected_string = Content::seleted_string(&selected_elements);

        match dialog::input_dialog(" Rename Dataset: ", "Enter the new name for the Dataset", selected_string.as_str()) {
            Ok(new_dataset_name) => { command::zfs_rename(selected_string, new_dataset_name); },
            Err(_)    => { },
        }

        dialog::refresh_screen();
    }
}