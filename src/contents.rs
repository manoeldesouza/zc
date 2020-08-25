
use crate::dialogs;

pub enum ContentType {
    Pools,
    Datasets,
    Volumes,
    Snapshots,
}

pub struct Content {
    pub is_selected:  bool,
    pub content_type: ContentType,
    pub position:     i32,
    pub start:        i32,
    pub list:         Vec<String>,
}

impl Content {

    pub fn new(is_selected: bool, content_type: ContentType) -> Content {

        Content {
            is_selected,
            content_type,
            position: 0,
            start: 0,
            list: Vec::new(),
        }
    }

    pub fn update(&mut self) {

        match self.content_type {
            ContentType::Pools     => { self.list = zpool::pool_list(); },
            ContentType::Datasets  => { self.list = zfs::dataset_list(); },
            ContentType::Volumes   => { self.list = zfs::volume_list(); },
            ContentType::Snapshots => { self.list = zfs::snapshot_list(); },
        }
    }

    pub fn next(&mut self) {

        let next_type = match self.content_type {
            ContentType::Pools     => { ContentType::Datasets  },
            ContentType::Datasets  => { ContentType::Volumes   },
            ContentType::Volumes   => { ContentType::Snapshots },
            ContentType::Snapshots => { ContentType::Pools     },
        };

        self.content_type = next_type;
        self.position = 0;
        self.start = 0;
        self.list = Vec::new();
    }

    pub fn title(&self) -> String {
        match self.content_type {
            ContentType::Pools     => { String::from("Pools")     },
            ContentType::Datasets  => { String::from("Datasets")  },
            ContentType::Volumes   => { String::from("Volumes")   },
            ContentType::Snapshots => { String::from("Snapshots") },
        }
    }

    pub fn jump_to(&mut self, position: i32) {

        if position < 0 || self.list.is_empty() {
            self.position = 0   

        } else if position > self.list.len() as i32 - 1 { 
            self.jump_to_last();

        } else {
            self.position = position;
        }    
    }

    pub fn jump(&mut self, elements: i32) {
        let position = self.position + elements;
        self.jump_to(position);
    }   

    pub fn jump_to_last(&mut self) {    
        let position = (self.list.len() as i32) - 1;
        self.jump_to(position);
    }

    pub fn scroll(&mut self, height: i32) {

        if self.position < 0     { self.start = 0; }   
        if self.start < 0        { self.start = 0; }

        if self.position > self.list.len() as i32 - 1 { self.position = self.list.len() as i32 - 1; }
        if self.position < self.start                 { self.start = self.position; }   
        if self.position >= self.start + height - 1   { self.start = self.position - height + 2; }   
    }

    pub fn help() {

        let title = "ZC - ZFS Commander";
        let prompt = "Help: ";
        
        let help = format!("{}\n{}", crate::HELP, crate::LICENSE)
            .lines()
            .map(|s: &str| s.to_string())
            .collect::<Vec<String>>();

        dialogs::presentation_box(title, prompt, help);
    }

    pub fn key_f(&mut self, function_key: i32) {

        let selected_value = match self.list.get(self.position as usize) {
            Some(value) => value,
            _     => "",
        };
    
        match function_key {
    
            1 => { Content::help(); },
    
            2 => {
                match self.content_type {
                    ContentType::Pools =>     { },
                    ContentType::Datasets =>  { zfs::dataset_promote(selected_value);  },
                    ContentType::Volumes =>   { },
                    ContentType::Snapshots => { zfs::snapshot_diff(selected_value);    },
                }
            },
    
            3 => { 
                match self.content_type {
                    ContentType::Pools =>     { },
                    ContentType::Datasets =>  { },
                    ContentType::Volumes =>   { },
                    ContentType::Snapshots => { zfs::snapshot_send(selected_value);    },    
                }
            },
    
            4 => { 
                match self.content_type {
                    ContentType::Pools =>     { },
                    ContentType::Datasets =>  { },
                    ContentType::Volumes =>   { },
                    ContentType::Snapshots => { },
                }
            },
    
            5 => { 
                match self.content_type {
                    ContentType::Pools =>     { },
                    ContentType::Datasets =>  { zfs::dataset_snapshot(selected_value); },
                    ContentType::Volumes =>   { zfs::dataset_snapshot(selected_value); },
                    ContentType::Snapshots => { zfs::snapshot_clone(selected_value);   },
                }
            },
    
            6 => { 
                match self.content_type {
                    ContentType::Pools =>     { },
                    ContentType::Datasets =>  { zfs::dataset_rename(selected_value);    },
                    ContentType::Volumes =>   { zfs::dataset_rename(selected_value);    },
                    ContentType::Snapshots => { zfs::dataset_rename(selected_value);    },
                }
            },
    
            7 => { 
                match self.content_type {
                    ContentType::Pools =>     { zpool::pool_scrub(selected_value);      },
                    ContentType::Datasets =>  { zfs::dataset_create(selected_value);    },
                    ContentType::Volumes =>   { zfs::volume_create(selected_value);     },
                    ContentType::Snapshots => { zfs::snapshot_rollback(selected_value); },
                }
            },
    
            8 => { 
                match self.content_type {
                    ContentType::Pools =>     { zpool::pool_destroy(selected_value);    },
                    ContentType::Datasets =>  { zfs::dataset_destroy(selected_value);   },
                    ContentType::Volumes =>   { zfs::dataset_destroy(selected_value);   },
                    ContentType::Snapshots => { zfs::dataset_destroy(selected_value);   },
                }
            },
    
            9 => { 
                match self.content_type {
                    ContentType::Pools =>     { zpool::pool_get_all(selected_value);    },
                    ContentType::Datasets =>  { zfs::dataset_get_all(selected_value);   },
                    ContentType::Volumes =>   { zfs::dataset_get_all(selected_value);   },
                    ContentType::Snapshots => { zfs::dataset_get_all(selected_value);   },
                }
            },
    
            11 => { },
    
            12 => { },
    
            _ => { },
        }
    }
}

mod zpool {

    use crate::commands;
    use crate::dialogs;

    pub fn pool_list() -> Vec<String> {
        commands::list("zpool", &["list", "-H", "-o", "name"])
    }

    pub fn pool_destroy(selected_value: &str) {

        let title = "Destroy Pool";
        let prompt = "The following pool will be destroyed: ";
        let message = vec![String::from(selected_value)];
    
        let err_title = "Error";
        let err_prompt = "Error during zpool destroy";
    
        let is_confirmed = dialogs::confirmation_box(title, prompt, message);
    
        if is_confirmed {
            let arguments = vec!["destroy", selected_value];
            let result = commands::list("zpool", &arguments);
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        } 
    }
    
    pub fn pool_scrub(selected_value: &str) {
    
        let title = "Scrub Pool";
        let prompt = "The following pool will be scrubbed: ";
        let message = vec![String::from(selected_value)];
    
        let err_title = "Error";
        let err_prompt = "Error during zpool scrub";
    
        let is_confirmed = dialogs::confirmation_box(title, prompt, message);
    
        if is_confirmed {
            let arguments = vec!["scrub", selected_value];
            let result = commands::list("zpool", &arguments);
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        } 
    }
    
    pub fn pool_get_all(selected_value: &str) {

        let title = "ZPOOL Get All";
        let err_title = "Error";
        let err_prompt = "Error during zpool set";

        let arguments = vec!["get", "all", selected_value];
        let result = commands::list("zpool", &arguments);    

        let clone = result.clone().get(1..).unwrap().to_vec();
        let legend = result.get(0).unwrap();

        if let Ok(selection) = dialogs::navigation_box(title, &legend, clone) {

            let title = "ZPOOL Set";
            let p_name: String;
            let p_type: String;
            let p_value: String;
            let value: String;

            if !selection.is_empty() {
                let mut p = selection.split_whitespace();

                p_name  = p.next().unwrap().to_string();
                p_type  = p.next().unwrap().to_string();
                p_value = p.next().unwrap().to_string();

                let prompt = format!("{}:{}", p_name, p_type);
                value = dialogs::single_input_box(title, &prompt, p_value);

            } else {
                let prompt = String::from("Enter property and value:");
                let results = dialogs::dual_input_box(title, &prompt, "property".to_string(), "value".to_string());
                p_type = results.0;
                value = results.1;
            }

            if !value.is_empty() {
                
                let property_value = format!("{}={}", p_type, value);
                let arguments = vec!["set", &property_value, &selected_value];
                let result = commands::list("zpool", &arguments);
            
                if !result.is_empty() {
                    dialogs::message_box(err_title, err_prompt, result);
                }    
            }
        }
    }
}

mod zfs {

    use crate::commands;
    use crate::dialogs;
    
    pub fn volume_list() -> Vec<String> {
        commands::list("zfs",   &["list", "-H", "-o", "name", "-t", "volume"])
    }

    pub fn volume_create(selected_value: &str) {
    
        let title = "Create Volume";
        let prompt = "Enter the name and size of the new volume: ";
        let default_value = format!("{}", selected_value);
    
        let err_title = "Error";
        let err_prompt = "Error during zfs create";
        let default_value2 = format!("1g");
    
        let (new_dataset1, size) = dialogs::dual_input_box(title, prompt, default_value, default_value2);
    
        if !new_dataset1.is_empty() {
            let arguments = vec!["create", "-V", &size, &new_dataset1];
            let result = commands::list("zfs", &arguments);    
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        }
    }
    
    pub fn dataset_list() -> Vec<String> {
        commands::list("zfs",   &["list", "-H", "-o", "name", "-t", "filesystem"])
    }

    pub fn dataset_destroy(selected_value: &str) {
    
        let title = "Destroy Dataset";
        let prompt = "The following dataset will be destroyed: ";
        let message = vec![String::from(selected_value)];
    
        let err_title = "Error";
        let err_prompt = "Error during zfs destroy";
    
        let is_confirmed = dialogs::confirmation_box(title, prompt, message);
    
        if is_confirmed {
            let arguments = vec!["destroy", selected_value];
            let result = commands::list("zfs", &arguments);    
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        }    
    }
    
    pub fn dataset_create(selected_value: &str) {
    
        let title = "Create Dataset";
        let prompt = "Enter the name of the new dataset: ";
        let default_value = format!("{}/", selected_value);
    
        let err_title = "Error";
        let err_prompt = "Error during zfs create";
    
        let new_dataset = dialogs::single_input_box(title, prompt, default_value);
    
        if !new_dataset.is_empty() {
            let arguments = vec!["create", &new_dataset];
            let result = commands::list("zfs", &arguments);    
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        }    
    }
    
    pub fn dataset_snapshot(selected_value: &str) {
    
        let title = "Snapshot Dataset";
        let prompt = "Enter the name of the new snapshot: ";
        let default_value = format!("{}@", selected_value);
    
        let err_title = "Error";
        let err_prompt = "Error during zfs snapshot";
    
        let new_dataset = dialogs::single_input_box(title, prompt, default_value);
    
        if !new_dataset.is_empty() {
            let arguments = vec!["snapshot", &new_dataset];
            let result = commands::list("zfs", &arguments);    
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        }   
    }
    
    pub fn dataset_rename(selected_value: &str) {
    
        let title = "Rename Dataset";
        let prompt = "Enter the new name for the dataset: ";
        let default_value = format!("{}", selected_value);
    
        let err_title = "Error";
        let err_prompt = "Error during zfs rename";
    
        let new_dataset = dialogs::single_input_box(title, prompt, default_value);
    
        if !new_dataset.is_empty() {
            let arguments = vec!["rename", selected_value, &new_dataset];
            let result = commands::list("zfs", &arguments);    
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        }   
    }
    
    pub fn dataset_promote(selected_value: &str) {
    
        let title = "Promote Dataset";
        let prompt = "Promote the following dataset: ";
        let default_value = vec![String::from(selected_value)];
    
        let err_title = "Error";
        let err_prompt = "Error during zfs promote";
    
        let is_confirmed = dialogs::confirmation_box(title, prompt, default_value);
    
        if is_confirmed {
            let arguments = vec!["promote", selected_value];
            let result = commands::list("zfs", &arguments);    
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        }     
    }
    
    pub fn dataset_get_all(selected_value: &str) {

        let title = "ZFS Get All";
        let err_title = "Error";
        let err_prompt = "Error during zfs set";

        let arguments = vec!["get", "all", selected_value];
        let result = commands::list("zfs", &arguments);    

        let clone = result.clone().get(1..).unwrap().to_vec();
        let legend = result.get(0).unwrap();

        if let Ok(selection) = dialogs::navigation_box(title, &legend, clone) {

            let title = "ZFS Set";
            let p_name: String;
            let p_type: String;
            let p_value: String;
            let value: String;

            if !selection.is_empty() {
                let mut p = selection.split_whitespace();

                p_name  = p.next().unwrap().to_string();
                p_type  = p.next().unwrap().to_string();
                p_value = p.next().unwrap().to_string();

                let prompt = format!("{}:{}", p_name, p_type);
                value = dialogs::single_input_box(title, &prompt, p_value);

            } else {
                let prompt = String::from("Enter property and value:");
                let results = dialogs::dual_input_box(title, &prompt, "property".to_string(), "value".to_string());
                p_type = results.0;
                value = results.1;
            }

            if !value.is_empty() {

                let property_value = format!("{}={}", p_type, value);
                let arguments = vec!["set", &property_value, &selected_value];
                let result = commands::list("zfs", &arguments);
            
                if !result.is_empty() {
                    dialogs::message_box(err_title, err_prompt, result);
                }    
            }
        }
    }
    
    pub fn snapshot_list() -> Vec<String> {
        commands::list("zfs",   &["list", "-H", "-o", "name", "-t", "snapshot"])
    }

    pub fn snapshot_clone(selected_value: &str) {
    
        let title = "Clone Snapshot";
        let prompt = "Enter the name of the dataset to be cloned from snapshot: ";
        let default_value = format!("");
    
        let err_title = "Error";
        let err_prompt = "Error during zfs clone";
    
        let new_dataset = dialogs::single_input_box(title, prompt, default_value);
    
        if !new_dataset.is_empty() {
            let arguments = vec!["clone", &selected_value, &new_dataset];
            let result = commands::list("zfs", &arguments);    
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        }   
    }
    
    pub fn snapshot_rollback(selected_value: &str) {
    
        let title = "Rollback to Snapshot";
        let prompt = "The dataset will be rolled-back to the following snapshot: ";
        let message = vec![String::from(selected_value)];
    
        let err_title = "Error";
        let err_prompt = "Error during zfs rollback";
    
        let is_confirmed = dialogs::confirmation_box(title, prompt, message);
    
        if is_confirmed {
            let arguments = vec!["rollback", selected_value];
            let result = commands::list("zfs", &arguments);    
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        }   
    }
    
    pub fn snapshot_diff(selected_value: &str) {
    
        let title = "Diff Snapshot";
        let prompt = format!("Compare snapshot {} with:", selected_value);
        let default_value = format!("{}", selected_value);
        let legend = "[ M modified | - removed | + created | R renamed ]";
    
        let new_dataset = dialogs::single_input_box(title, &prompt, default_value);
    
        if !new_dataset.is_empty() {
            let arguments = vec!["diff", &selected_value, &new_dataset];
            let result = commands::list("zfs", &arguments);    
    
            dialogs::presentation_box(title, legend, result);
        }
    }
    
    pub fn snapshot_send(selected_value: &str) {
    
        let title = "Send Snapshot";
        let prompt = "Enter the Snapshot and stream to send: ";
        let default_value = format!("{}", selected_value);
    
        let err_title = "Error";
        let err_prompt = "Error during zfs send";
        let default_value2 = format!("zfs recv pool/dataset");
    
        let (send_snapshot, stream) = dialogs::dual_input_box(title, prompt, default_value, default_value2);
    
        if !send_snapshot.is_empty() {
            let send_args = vec!["send", &send_snapshot];
            let stream_str: Vec<&str> = stream.split_whitespace().collect();

            let recv_cmd = stream_str.get(0).unwrap();
            let recv_args = stream_str.get(1..).unwrap().to_vec();

            let result = commands::piped("zfs", send_args, recv_cmd, recv_args);    
    
            if !result.is_empty() {
                dialogs::message_box(err_title, err_prompt, result);
            }
        }
    }
}
