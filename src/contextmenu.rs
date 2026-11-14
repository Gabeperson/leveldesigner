use super::*;

#[derive(Debug, Clone, Default)]
pub enum ContextMenuTarget {
    SingleObject {
        id: u32,
    },
    #[default]
    MultiObjects,
    Waypoint {
        id: u32,
        waypoint_id: String,
    },
    Bird {
        id: u32,
    },
}

impl ContextMenuTarget {
    pub fn options(&self) -> Vec<String> {
        match self {
            ContextMenuTarget::SingleObject { .. } => vec![
                "Send backward".into(),
                "Send forward".into(),
                "Send to back".into(),
                "Send to front".into(),
            ],
            ContextMenuTarget::Waypoint {
                waypoint_id: buffer,
                ..
            } => vec![
                "Send backward".into(),
                "Send forward".into(),
                "Send to back".into(),
                "Send to front".into(),
                format!(
                    "ID: {}",
                    if buffer.is_empty() {
                        String::from("_")
                    } else {
                        buffer.clone()
                    }
                ),
            ],
            ContextMenuTarget::MultiObjects => {
                vec!["Send to back".into(), "Send to front".into()]
            }
            ContextMenuTarget::Bird { .. } => {
                vec![
                    "Send backward".into(),
                    "Send forward".into(),
                    "Send to back".into(),
                    "Send to front".into(),
                    "Regular".into(),
                    "Pathfinding".into(),
                    "CanLeave".into(),
                    "CannotLeave".into(),
                ]
            }
        }
    }
    pub fn apply(&self, app: &mut App, n: usize) -> bool {
        match self {
            ContextMenuTarget::SingleObject { id } => match n {
                0 => app.send_backward(*id),
                1 => app.send_forward(*id),
                2 => app.send_to_back_single(*id),
                3 => app.send_to_front_single(*id),
                _ => {}
            },
            ContextMenuTarget::MultiObjects => match n {
                0 => app.send_to_back_multi(),
                1 => app.send_to_front_multi(),
                _ => {}
            },
            ContextMenuTarget::Waypoint { id, waypoint_id } => match n {
                0 => app.send_backward(*id),
                1 => app.send_forward(*id),
                2 => app.send_to_back_single(*id),
                3 => app.send_to_front_single(*id),
                4 => {
                    let Ok(v) = waypoint_id.parse::<i32>() else {
                        return false;
                    };
                    let Some(obj) = app.objects.get_mut(id) else {
                        return true;
                    };
                    let Object::WaypointMarker { waypoint_id, .. } = obj else {
                        return false;
                    };
                    *waypoint_id = v;
                }
                _ => {}
            },
            ContextMenuTarget::Bird { id } => match n {
                0 => app.send_backward(*id),
                1 => app.send_forward(*id),
                2 => app.send_to_back_single(*id),
                3 => app.send_to_front_single(*id),
                4 => {
                    let Some(obj) = app.objects.get_mut(id) else {
                        return true;
                    };
                    let Object::Bird { kind, .. } = obj else {
                        return false;
                    };
                    *kind = BirdKind::Regular
                }
                5 => {
                    let Some(obj) = app.objects.get_mut(id) else {
                        return true;
                    };
                    let Object::Bird { kind, .. } = obj else {
                        return false;
                    };
                    *kind = BirdKind::Pathfinding
                }
                6 => {
                    let Some(obj) = app.objects.get_mut(id) else {
                        return true;
                    };
                    let Object::Bird {
                        can_leave_spawn, ..
                    } = obj
                    else {
                        return false;
                    };
                    *can_leave_spawn = true;
                }
                7 => {
                    let Some(obj) = app.objects.get_mut(id) else {
                        return true;
                    };
                    let Object::Bird {
                        can_leave_spawn, ..
                    } = obj
                    else {
                        return false;
                    };
                    *can_leave_spawn = false
                }
                _ => {}
            },
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct ContextMenu {
    pub pos: Vec2,
    pub target: ContextMenuTarget,
}

impl App {
    pub fn context_menu(&mut self) {
        let Some(ContextMenu { pos, target }) = &mut self.context_menu else {
            return;
        };

        let items = target.options();
        let contextmenu_size: Vec2 =
            Vec2::new(CONTEXT_MENU_HEIGHT, ITEM_HEIGHT * items.len() as f32);
        let menu = Rect::new(pos.x, pos.y, contextmenu_size.x, contextmenu_size.y);
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        let esc_pressed = is_key_pressed(KeyCode::Escape);
        let raw_mouse_pos = {
            let pos = mouse_position();
            Vec2::new(pos.0, pos.1)
        };
        if esc_pressed || (mouse_pressed && !menu.contains(raw_mouse_pos)) {
            self.context_menu = None;
            return;
        }
        if let ContextMenuTarget::Waypoint { waypoint_id, .. } = target {
            while let Some(c) = get_char_pressed() {
                if c.is_ascii_digit() || c == '-' {
                    waypoint_id.push(c);
                }
            }
        };
        draw_rectangle_ex(
            menu.x,
            menu.y,
            menu.w,
            menu.h,
            DrawRectangleParams {
                color: WHITE,
                ..Default::default()
            },
        );
        let mut start = pos.y;
        for (i, text) in items.iter().enumerate() {
            let rect = Rect::new(pos.x, start, menu.w, ITEM_HEIGHT);
            start += ITEM_HEIGHT;
            let hovered = rect.contains(raw_mouse_pos);
            if hovered {
                draw_rectangle_ex(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    DrawRectangleParams {
                        color: Color::from_rgba(230, 240, 255, 255),
                        ..Default::default()
                    },
                )
            }
            let font_size = 20.0;
            let dims = measure_text(text, None, font_size as u16, 1.0);
            let x = rect.x + 10.;
            let y = rect.y + rect.h / 2. + dims.height / 2.;
            draw_text(text, x, y, font_size, BLACK);
            if mouse_pressed && hovered {
                let target = std::mem::take(target);
                target.apply(self, i);
                self.context_menu = None;
                break;
            }
        }
        draw_rectangle_lines_ex(
            menu.x,
            menu.y,
            menu.w,
            menu.h,
            1.0,
            DrawRectangleParams {
                color: DARKGRAY,
                ..Default::default()
            },
        );
    }
}
