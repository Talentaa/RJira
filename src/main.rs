mod db;
mod io_utils;
mod models;
mod navigator;
mod ui;

use std::rc::Rc;

use db::JiraDatabase;
use io_utils::*;
use navigator::*;

fn main() {
    let db = Rc::new(JiraDatabase::new("data/db.json".to_string()));
    let mut nav = Navigator::new(db);

    loop {
        clearscreen::clear().unwrap();
        
        if let Some(page) = nav.get_current_page() {
            if let Err(error) = page.draw_page() {
                error_handle(error);
            }

            let input = get_user_input();
            match page.handle_input(input.trim()) {
                Ok(Some(action)) => {
                    if let Err(error) = nav.handle_action(action) {
                        error_handle(error);
                    }
                }
                Ok(None) => {}
                Err(error) => {
                    error_handle(error);
                }
            };
        } else {
            break;
        }

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}

fn error_handle(error: anyhow::Error) {
    println!("Error rendering page: {}", error);
    println!("Press any key to continue...");
    wait_for_key_press();
}
