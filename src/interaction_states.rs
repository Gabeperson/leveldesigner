use super::*;
#[derive(Debug, Clone)]
pub enum InteractionState {
    None,
    Selecting {
        start: Vec2,
    },
    BeforeMoving {
        initial_positions: RefCell<HashMap<u32, Vec2>>,
        mouse_start: Vec2,
    },
    Moving {
        initial_positions: HashMap<u32, Vec2>,
        mouse_start: Vec2,
    },
    MovingRailStart {
        id: u32,
    },
    MovingRailEnd {
        id: u32,
    },
    MoveLaserBase {
        id: u32,
    },
    DetectionRange {
        id: u32,
    },
    ConeAngleRadius {
        id: u32,
    },
    ConeWidth {
        id: u32,
    },
    Resizing {
        handle: HandleKind,
        initial_group_rect: Rect,
        initial_object_rects: HashMap<u32, Rect>,
        mouse_start: Vec2,
    },
    AddingRectSelected {
        kind: RectKind,
    },
    AddingRectDragging {
        kind: RectKind,
        start: Vec2,
    },
    AddingObject {
        object: Object,
    },
}
impl App {
    pub fn state_update(&mut self) {
        let raw_mouse_pos = {
            let pos = mouse_position();
            Vec2::new(pos.0, pos.1)
        };
        let camera = self.camera();
        set_camera(&camera);
        let mouse_pos_world = camera.screen_to_world(raw_mouse_pos);
        let mouse_down = is_mouse_button_down(MouseButton::Left);
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        let mouse_right_pressed = is_mouse_button_pressed(MouseButton::Right);
        let shift_down = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let delete = is_key_pressed(KeyCode::Delete) || is_key_pressed(KeyCode::Backspace);
        let escape = is_key_pressed(KeyCode::Escape);
        let dupe = (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
            && is_key_pressed(KeyCode::D);
        let copy = (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
            && is_key_pressed(KeyCode::C);
        let paste = (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
            && is_key_pressed(KeyCode::V);
        let group_rect = get_group_rect(&self.objects, &self.selected_ids);
        match &mut self.state {
            InteractionState::None => {
                if delete {
                    Self::push_snapshot(
                        &self.ordering,
                        &self.objects,
                        &self.selected_ids,
                        &mut self.undo_snapshots,
                        &mut self.redo_snapshots,
                    );
                    for id in &self.selected_ids {
                        self.objects.remove(id);
                    }
                    self.ordering.retain(|id| !self.selected_ids.contains(id));
                    self.selected_ids.clear();
                } else if dupe {
                    self.dupe();
                } else if copy {
                    self.clipboard = self.copy();
                } else if paste {
                    let x = screen_width() / 2.;
                    let y = screen_height() / 2.;
                    let xy = camera.screen_to_world(Vec2::new(x, y));
                    self.paste(xy);
                } else if mouse_pressed {
                    let mut clicked_handle = None;
                    match self.selected_ids.len() {
                        0 => {}
                        1 => {
                            let id = self.selected_ids.iter().next().unwrap();
                            let object = &self.objects[id];
                            clicked_handle = get_active_handle(
                                object,
                                mouse_pos_world,
                                HANDLE_SIZE / self.camera_zoom,
                            );
                        }
                        _ => {
                            let g_rect = group_rect.unwrap();
                            clicked_handle = get_active_handle_group(
                                g_rect,
                                mouse_pos_world,
                                HANDLE_SIZE / self.camera_zoom,
                            );
                        }
                    }
                    if let Some(handle) = clicked_handle {
                        Self::push_snapshot(
                            &self.ordering,
                            &self.objects,
                            &self.selected_ids,
                            &mut self.undo_snapshots,
                            &mut self.redo_snapshots,
                        );
                        let id = *self.selected_ids.iter().next().unwrap();
                        match handle {
                            HandleKind::TopLeft
                            | HandleKind::TopMiddle
                            | HandleKind::TopRight
                            | HandleKind::RightMiddle
                            | HandleKind::BottomRight
                            | HandleKind::BottomMiddle
                            | HandleKind::BottomLeft
                            | HandleKind::LeftMiddle => {
                                let g_rect = group_rect.unwrap();
                                let mut initial_object_rects = HashMap::new();
                                for &id in &self.selected_ids {
                                    if let Some(obj) = self.objects.get(&id) {
                                        initial_object_rects.insert(id, obj.aabb());
                                    }
                                }
                                self.state = InteractionState::Resizing {
                                    handle,
                                    initial_group_rect: g_rect,
                                    initial_object_rects,
                                    mouse_start: mouse_pos_world,
                                };
                            }
                            HandleKind::RailStart => {
                                self.state = InteractionState::MovingRailStart { id }
                            }
                            HandleKind::RailEnd => {
                                self.state = InteractionState::MovingRailEnd { id }
                            }
                            HandleKind::MoveLaserBase => {
                                self.state = InteractionState::MoveLaserBase { id }
                            }
                            HandleKind::DetectionRange => {
                                self.state = InteractionState::DetectionRange { id }
                            }
                            HandleKind::ConeWidth => {
                                self.state = InteractionState::ConeWidth { id }
                            }
                            HandleKind::ConeAngleRadius => {
                                self.state = InteractionState::ConeAngleRadius { id }
                            }
                        }
                    } else {
                        let mut clicked_id = None;
                        for id in self.ordering.iter().rev() {
                            let obj = self.objects[id];
                            if obj.contains(mouse_pos_world) {
                                clicked_id = Some(*id);
                                break;
                            }
                        }

                        if let Some(id) = clicked_id {
                            if shift_down {
                                if self.selected_ids.contains(&id) {
                                    self.selected_ids.remove(&id);
                                } else {
                                    self.selected_ids.insert(id);
                                }
                            } else {
                                if !self.selected_ids.contains(&id) {
                                    self.selected_ids.clear();
                                    self.selected_ids.insert(id);
                                }
                            }

                            let mut initial_positions = HashMap::new();
                            for &sel_id in &self.selected_ids {
                                if let Some(obj) = self.objects.get(&sel_id) {
                                    initial_positions.insert(sel_id, obj.pos());
                                }
                            }
                            self.state = InteractionState::BeforeMoving {
                                initial_positions: RefCell::new(initial_positions),
                                mouse_start: mouse_pos_world,
                            };
                        } else {
                            if !shift_down {
                                self.selected_ids.clear();
                            }
                            self.state = InteractionState::Selecting {
                                start: mouse_pos_world,
                            };
                        }
                    }
                }
                if mouse_right_pressed {
                    let mut clicked_id = None;
                    for id in self.ordering.iter().rev() {
                        let obj = self.objects[id];
                        if obj.contains(mouse_pos_world) {
                            clicked_id = Some(*id);
                            break;
                        }
                    }
                    if let Some(id) = clicked_id {
                        let target =
                            if self.selected_ids.contains(&id) && self.selected_ids.len() != 1 {
                                ContextMenuTarget::MultiObjects
                            } else {
                                let obj = self.objects[&id];
                                obj.context_menu_target(id)
                            };
                        self.context_menu = Some(ContextMenu {
                            pos: raw_mouse_pos,
                            target,
                        })
                    }
                }
            }
            InteractionState::Selecting { start } => {
                if !mouse_down {
                    let sel_rect = Rect::new(
                        start.x.min(mouse_pos_world.x),
                        start.y.min(mouse_pos_world.y),
                        (start.x - mouse_pos_world.x).abs(),
                        (start.y - mouse_pos_world.y).abs(),
                    );

                    if !shift_down {
                        self.selected_ids.clear();
                    }

                    for (&id, obj) in &self.objects {
                        if obj.overlaps_rect(sel_rect) {
                            self.selected_ids.insert(id);
                        }
                    }
                    self.state = InteractionState::None;
                }
            }
            InteractionState::BeforeMoving {
                initial_positions,
                mouse_start,
            } => {
                if mouse_down {
                    let delta = mouse_pos_world - *mouse_start;
                    if delta.length() > 0.1 {
                        Self::push_snapshot(
                            &self.ordering,
                            &self.objects,
                            &self.selected_ids,
                            &mut self.undo_snapshots,
                            &mut self.redo_snapshots,
                        );

                        for (&id, &init_pos) in &*initial_positions.borrow() {
                            if let Some(obj) = self.objects.get_mut(&id) {
                                obj.set_pos(init_pos + delta);
                            }
                        }
                        self.state = InteractionState::Moving {
                            initial_positions: initial_positions.take(),
                            mouse_start: *mouse_start,
                        }
                    }
                } else {
                    self.state = InteractionState::None;
                }
            }
            InteractionState::Moving {
                initial_positions,
                mouse_start,
            } => {
                if mouse_down {
                    let delta = mouse_pos_world - *mouse_start;
                    for (&id, &mut init_pos) in initial_positions {
                        if let Some(obj) = self.objects.get_mut(&id) {
                            obj.set_pos(init_pos + delta);
                        }
                    }
                } else {
                    self.state = InteractionState::None;
                }
            }
            InteractionState::Resizing {
                handle,
                initial_group_rect,
                initial_object_rects,
                mouse_start,
            } => {
                if mouse_down {
                    let delta = mouse_pos_world - *mouse_start;
                    let new_group_rect = resized(*initial_group_rect, *handle, delta);

                    let scale_x = if initial_group_rect.w > 0.0 {
                        new_group_rect.w / initial_group_rect.w
                    } else {
                        1.0
                    };
                    let scale_y = if initial_group_rect.h > 0.0 {
                        new_group_rect.h / initial_group_rect.h
                    } else {
                        1.0
                    };

                    for (&id, init_rect) in initial_object_rects {
                        if let Some(obj) = self.objects.get_mut(&id) {
                            let rel_x = init_rect.x - initial_group_rect.x;
                            let rel_y = init_rect.y - initial_group_rect.y;

                            let x = new_group_rect.x + rel_x * scale_x;
                            let y = new_group_rect.y + rel_y * scale_y;
                            let w = init_rect.w * scale_x;
                            let h = init_rect.h * scale_y;

                            obj.set_pos(Vec2::new(x, y));
                            obj.set_size(Vec2::new(w, h));
                        }
                    }
                } else {
                    self.state = InteractionState::None;
                }
            }
            InteractionState::AddingRectSelected { kind } => {
                if mouse_pressed {
                    Self::push_snapshot(
                        &self.ordering,
                        &self.objects,
                        &self.selected_ids,
                        &mut self.undo_snapshots,
                        &mut self.redo_snapshots,
                    );
                    self.selected_ids.clear();
                    self.state = InteractionState::AddingRectDragging {
                        kind: *kind,
                        start: mouse_pos_world,
                    }
                }
            }
            InteractionState::AddingRectDragging { kind, start } => {
                let rect = Rect::new(
                    start.x.min(mouse_pos_world.x),
                    start.y.min(mouse_pos_world.y),
                    (start.x - mouse_pos_world.x).abs(),
                    (start.y - mouse_pos_world.y).abs(),
                );

                let show = rect.w > 2.0 && rect.h > 2.0;

                if !mouse_down {
                    if show {
                        let id = new_id();
                        self.objects.insert(id, Object::Rect { rect, kind: *kind });
                        self.ordering.push(id);
                        self.selected_ids.clear();
                        self.selected_ids.insert(id);
                    } else {
                        self.undo_snapshots.pop();
                    }
                    self.state = InteractionState::None;
                } else {
                    if show {
                        Object::Rect { rect, kind: *kind }.draw(&self.textures);
                    }
                }
            }
            InteractionState::MovingRailStart { id } => {
                if mouse_down {
                    let object = self.objects.get_mut(id).unwrap();
                    let Object::Laser {
                        rail_start,
                        rail_end,
                        ..
                    } = object
                    else {
                        unreachable!();
                    };
                    *rail_start = mouse_pos_world;
                    if (*rail_end - *rail_start).length() < 20. {
                        *rail_start = (mouse_pos_world - *rail_end).normalize_or(Vec2::ONE) * 20.;
                    }
                } else {
                    self.state = InteractionState::None;
                }
            }
            InteractionState::MovingRailEnd { id } => {
                if mouse_down {
                    let object = self.objects.get_mut(id).unwrap();
                    let Object::Laser {
                        rail_start,
                        rail_end,
                        ..
                    } = object
                    else {
                        unreachable!();
                    };
                    *rail_end = mouse_pos_world;
                    if (*rail_end - *rail_start).length() < 20. {
                        *rail_end = (mouse_pos_world - *rail_start).normalize_or(Vec2::ONE) * 20.;
                    }
                } else {
                    self.state = InteractionState::None;
                }
            }
            InteractionState::MoveLaserBase { id } => {
                if mouse_down {
                    let object = self.objects.get_mut(id).unwrap();
                    let Object::Laser {
                        rail_start,
                        rail_end,
                        rel,
                    } = object
                    else {
                        unreachable!();
                    };
                    let rail = *rail_end - *rail_start;
                    let mouse = mouse_pos_world - *rail_start;
                    let sqlen = rail.length_squared();
                    let p = (rail.dot(mouse) / sqlen).clamp(0.0, 1.0);
                    *rel = p;
                } else {
                    self.state = InteractionState::None;
                }
            }
            InteractionState::DetectionRange { id } => {
                if mouse_down {
                    let object = self.objects.get_mut(id).unwrap();
                    match object {
                        Object::Bird {
                            rect,
                            detection_range,
                            ..
                        } => {
                            let pos = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                            let diff = (pos - mouse_pos_world).length();
                            *detection_range = diff;
                        }
                        Object::WaypointMarker { .. }
                        | Object::Wave { .. }
                        | Object::Checkpoint { .. }
                        | Object::TutorialTexture { .. }
                        | Object::Heart { .. }
                        | Object::Banana { .. }
                        | Object::Strawberry { .. }
                        | Object::Rect { .. }
                        | Object::Laser { .. }
                        | Object::Player { .. } => {
                            self.state = InteractionState::None;
                        }
                    }
                } else {
                    self.state = InteractionState::None;
                }
            }
            InteractionState::AddingObject { object } => {
                object.set_pos_centered(mouse_pos_world);
                object.draw_with_alpha(&self.textures, 0.3);
                if escape || mouse_right_pressed {
                    self.state = InteractionState::None;
                } else if mouse_pressed {
                    let id = new_id();
                    self.objects.insert(id, *object);
                    self.ordering.push(id);
                    self.selected_ids.clear();
                    self.selected_ids.insert(id);
                    self.state = InteractionState::None;
                }
            }
            InteractionState::ConeAngleRadius { id } => {
                if mouse_down {
                    let object = self.objects.get_mut(id).unwrap();
                    match object {
                        Object::Wave {
                            rect,
                            radius,
                            angle,
                            width: _,
                        } => {
                            let pos = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                            let diff_v = mouse_pos_world - pos;
                            let diff = diff_v.length();
                            *radius = diff.max(10.);
                            *angle = diff_v.y.atan2(diff_v.x);
                        }
                        Object::WaypointMarker { .. }
                        | Object::Bird { .. }
                        | Object::Checkpoint { .. }
                        | Object::TutorialTexture { .. }
                        | Object::Heart { .. }
                        | Object::Banana { .. }
                        | Object::Strawberry { .. }
                        | Object::Rect { .. }
                        | Object::Laser { .. }
                        | Object::Player { .. } => {
                            self.state = InteractionState::None;
                        }
                    }
                } else {
                    self.state = InteractionState::None;
                }
            }
            InteractionState::ConeWidth { id } => {
                if mouse_down {
                    let object = self.objects.get_mut(id).unwrap();
                    match object {
                        Object::Wave {
                            rect,
                            radius: _,
                            angle,
                            width,
                        } => {
                            let pos = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                            let mousedir = mouse_pos_world - pos;
                            let dir = Vec2::from_angle(*angle);
                            let a = dir.angle_between(mousedir) * 2.;
                            *width = a.abs();
                        }
                        Object::WaypointMarker { .. }
                        | Object::Bird { .. }
                        | Object::Checkpoint { .. }
                        | Object::TutorialTexture { .. }
                        | Object::Heart { .. }
                        | Object::Banana { .. }
                        | Object::Strawberry { .. }
                        | Object::Rect { .. }
                        | Object::Laser { .. }
                        | Object::Player { .. } => {
                            self.state = InteractionState::None;
                        }
                    }
                } else {
                    self.state = InteractionState::None;
                }
            }
        }
        set_default_camera();
    }
}
