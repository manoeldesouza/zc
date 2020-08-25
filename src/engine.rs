
use crate::contents::{Content, ContentType};
use crate::dialogs;

pub struct Engine {
    left:  Content,
    right: Content,
}

impl Engine {

    pub fn new() -> Engine {

        dialogs::initialize();

        Engine {

            left:  Content::new(true,  ContentType::Datasets),
            right: Content::new(false, ContentType::Snapshots),
        }
    }

    pub fn run(&mut self) {

        loop {

            self.left.update();
            self.right.update();

            dialogs::dual_pane(&mut self.left, &mut self.right);
            let is_to_finish = dialogs::handle_keys(&mut self.left, &mut self.right);
            if is_to_finish { break; }
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        dialogs::finish();
    }    
}