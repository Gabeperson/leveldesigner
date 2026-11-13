use macroquad::prelude::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write;
use std::sync::atomic::{AtomicU32, Ordering};

mod app;
mod consts;
mod contextmenu;
mod handles;
mod interaction_states;
mod object;
mod ser_deser;
mod side_panel;
use app::*;
use consts::*;
use contextmenu::*;
use handles::*;
use interaction_states::*;
use object::*;

fn new_id() -> u32 {
    static ID: AtomicU32 = AtomicU32::new(1);
    ID.fetch_add(1, Ordering::Relaxed)
}

#[macroquad::main("Level design")]
async fn main() {
    let mut args = std::env::args();
    let _name = args.next().unwrap();
    let Some(file_name) = args.next() else {
        eprintln!("\x1b[31mError\x1b[0m: must give filename to open!");
        std::process::exit(1);
    };
    match std::fs::copy(&file_name, format!("{file_name}.backup")) {
        Ok(_) => {
            println!("Succesfully backed up file");
        }
        Err(e) => {
            eprintln!("\x1b[33mWarning\x1b[0m: Failed to back up file: {e}")
        }
    }
    if args.next().is_some() {
        eprintln!("\x1b[31mError\x1b[0m: Expected only 1 arg!");
        std::process::exit(1);
    }
    let mut app = App::new().await;
    ser_deser::parse(&file_name, &mut app);
    loop {
        clear_background(LIGHTGRAY);
        let raw_mouse_pos = {
            let pos = mouse_position();
            Vec2::new(pos.0, pos.1)
        };
        let in_sidebar = raw_mouse_pos.x <= SIDEBAR_SIZE;
        let in_menu = if let Some(ContextMenu { pos, target }) = &app.context_menu {
            let rect = Rect::new(
                pos.x,
                pos.y,
                CONTEXT_MENU_HEIGHT,
                target.options().len() as f32 * ITEM_HEIGHT,
            );
            rect.contains(raw_mouse_pos)
        } else {
            false
        };

        app.camera_zoom();
        app.camera_pan();
        app.undo_redo();
        if !in_sidebar && !in_menu {
            app.state_update();
        }
        app.render();
        app.side_panel();

        app.context_menu();

        if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
            && is_key_pressed(KeyCode::S)
        {
            match ser_deser::serialise(&app) {
                Ok(Some(serialised)) => {
                    if let Err(e) = std::fs::write(&file_name, serialised) {
                        eprintln!("Failed to save: {e}");
                    } else {
                        println!("Successfully saved to {file_name}");
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    eprintln!("\x1b[31mError\x1b[0m occured when serialising: {e}");
                }
            }
        }

        next_frame().await;
    }
}
