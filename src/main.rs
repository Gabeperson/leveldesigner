use macroquad::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RectKind {
    Regular,
    AntiTp,
    Tar,
    Hot,
}

#[derive(Clone, Copy, Debug)]
enum Object {
    Rect { rect: Rect, kind: RectKind },
}

impl Object {
    fn aabb(&self) -> Rect {
        match self {
            Object::Rect { rect, .. } => *rect,
        }
    }
    fn contains(&self, pos: Vec2) -> bool {
        match self {
            Object::Rect { rect, .. } => rect.contains(pos),
        }
    }
    fn pos(&self) -> Vec2 {
        match self {
            Object::Rect { rect, .. } => Vec2::new(rect.x, rect.y),
        }
    }
    fn set_pos(&mut self, pos: Vec2) {
        match self {
            Object::Rect { rect, .. } => {
                rect.x = pos.x;
                rect.y = pos.y;
            }
        }
    }
    fn size(&self) -> Vec2 {
        match self {
            Object::Rect { rect, .. } => Vec2::new(rect.w, rect.h),
        }
    }
    fn set_size(&mut self, size: Vec2) {
        match self {
            Object::Rect { rect, .. } => {
                rect.w = size.x;
                rect.h = size.y;
            }
        }
    }
    fn overlaps_rect(&self, rect: Rect) -> bool {
        match self {
            Object::Rect {
                rect: self_rect, ..
            } => self_rect.overlaps(&rect),
        }
    }
    fn draw(&self) {
        match self {
            Object::Rect { rect, kind } => {
                let (fill, outline) = match kind {
                    RectKind::Regular => (
                        Color::from_rgba(165, 216, 255, 255),
                        Color::new(0.5, 0.5, 0.5, 1.),
                    ),
                    RectKind::AntiTp => (
                        Color::new(1., 0.35, 0.9, 1.),
                        Color::new(0.65, 0.2, 0.5, 1.0),
                    ),
                    RectKind::Tar => (
                        Color::new(0.1, 0.1, 0.1, 1.0),
                        Color::new(0.5, 0.5, 0.5, 1.0),
                    ),
                    RectKind::Hot => (
                        Color::new(1.0, 0.8, 0.15, 1.0),
                        Color::new(1.0, 0.7, 0.15, 1.0),
                    ),
                };
                draw_rectangle_ex(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    DrawRectangleParams {
                        color: fill,
                        ..Default::default()
                    },
                );
                draw_rectangle_lines_ex(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    1.0,
                    DrawRectangleParams {
                        color: outline,
                        ..Default::default()
                    },
                );
            }
        }
    }

    fn context_menu_target(&self, id: u32) -> ContextMenuTarget {
        match self {
            Object::Rect { .. } => ContextMenuTarget::SingleObject { id },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum HandleKind {
    TopLeft,
    TopMiddle,
    TopRight,
    RightMiddle,
    BottomRight,
    BottomMiddle,
    BottomLeft,
    LeftMiddle,
}

#[derive(Debug, Clone)]
enum InteractionState {
    None,
    Selecting {
        start: Vec2,
    },
    Selected {
        initial_positions: RefCell<HashMap<u32, Vec2>>,
        mouse_start: Vec2,
    },
    Moving {
        initial_positions: HashMap<u32, Vec2>,
        mouse_start: Vec2,
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
}

const HANDLE_SIZE: f32 = 10.0;
const SIDEBAR_SIZE: f32 = 30.;
const PATCH_SIZE: f32 = 20.;

fn new_id() -> u32 {
    static ID: AtomicU32 = AtomicU32::new(1);
    ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone, Default)]
enum ContextMenuTarget {
    SingleObject {
        id: u32,
    },
    #[default]
    MultiObjects,
}

impl ContextMenuTarget {
    fn options(&self) -> &[&str] {
        match self {
            ContextMenuTarget::SingleObject { .. } => &[
                "Send backward",
                "Send forward",
                "Send to back",
                "Send to front",
            ],
            ContextMenuTarget::MultiObjects => &["Send to back", "Send to front"],
        }
    }
    fn apply(&self, app: &mut App, n: usize) {
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
        }
    }
}

#[derive(Debug, Clone)]
struct ContextMenu {
    pos: Vec2,
    target: ContextMenuTarget,
}

#[derive(Debug, Clone)]
struct Snapshot {
    ordering: Vec<u32>,
    objects: HashMap<u32, Object>,
    selected_ids: HashSet<u32>,
}

#[derive(Debug, Clone)]
struct App {
    state: InteractionState,
    ordering: Vec<u32>,
    objects: HashMap<u32, Object>,
    selected_ids: HashSet<u32>,
    undo_snapshots: Vec<Snapshot>,
    redo_snapshots: Vec<Snapshot>,
    context_menu: Option<ContextMenu>,
    camera_zoom: f32,
    camera_pos: Vec2,
    pan_start: Vec2,
}

impl App {
    fn new() -> Self {
        Self {
            state: InteractionState::None,
            ordering: Vec::new(),
            objects: HashMap::new(),
            selected_ids: HashSet::new(),
            undo_snapshots: Vec::new(),
            redo_snapshots: Vec::new(),
            context_menu: None,
            camera_zoom: 1.,
            camera_pos: Vec2::ZERO,
            pan_start: Vec2::ZERO,
        }
    }

    fn camera(&self) -> Camera2D {
        Camera2D {
            zoom: self.camera_zoom * Vec2::new(2.0 / screen_width(), -2.0 / screen_height()),
            target: self.camera_pos,
            ..Default::default()
        }
    }

    fn camera_zoom(&mut self) {
        let camera = self.camera();
        let raw_mouse_pos = {
            let pos = mouse_position();
            Vec2::new(pos.0, pos.1)
        };
        let wheel = mouse_wheel().1;
        if wheel != 0.0 && raw_mouse_pos.x > SIDEBAR_SIZE {
            let clamped = wheel.clamp(-3., 3.);
            let zoom = 1.1f32.powf(clamped);
            let mouse_before = camera.screen_to_world(raw_mouse_pos);
            self.camera_zoom *= zoom;
            self.camera_zoom = self.camera_zoom.clamp(0.05, 20.);
            let updated = self.camera();
            let mouse_after = updated.screen_to_world(raw_mouse_pos);
            self.camera_pos += mouse_before - mouse_after;
        }
    }

    fn camera_pan(&mut self) -> bool {
        let camera = self.camera();
        let middle_mouse_down = is_mouse_button_down(MouseButton::Middle);
        let middle_mouse_pressed = is_mouse_button_pressed(MouseButton::Middle);
        let raw_mouse_pos = {
            let pos = mouse_position();
            Vec2::new(pos.0, pos.1)
        };

        if middle_mouse_pressed {
            self.pan_start = camera.screen_to_world(raw_mouse_pos);
        }
        if middle_mouse_down {
            let current = camera.screen_to_world(raw_mouse_pos);
            self.camera_pos += self.pan_start - current;
            true
        } else {
            false
        }
    }

    fn send_to_back_single(&mut self, id: u32) {
        if let Some(pos) = self.ordering.iter().position(|&x| x == id) {
            let id = self.ordering.remove(pos);
            self.ordering.insert(0, id);
        }
    }
    fn send_to_front_single(&mut self, id: u32) {
        if let Some(pos) = self.ordering.iter().position(|&x| x == id) {
            let id = self.ordering.remove(pos);
            self.ordering.push(id);
        }
    }

    fn send_to_back_multi(&mut self) {
        let mut selected = Vec::new();
        self.ordering.retain(|id| {
            if self.selected_ids.contains(id) {
                selected.push(*id);
                false
            } else {
                true
            }
        });
        self.ordering.splice(0..0, selected);
    }

    fn send_to_front_multi(&mut self) {
        let mut selected = Vec::new();
        self.ordering.retain(|id| {
            if self.selected_ids.contains(id) {
                selected.push(*id);
                false
            } else {
                true
            }
        });
        self.ordering.extend(selected);
    }

    fn send_backward(&mut self, id: u32) {
        let Some(index) = self.ordering.iter().position(|&x| x == id) else {
            return;
        };
        if index == 0 {
            return;
        }
        let Some(obj) = self.objects.get(&id) else {
            return;
        };
        let aabb = obj.aabb();
        let mut target = index - 1;
        for j in (0..index).rev() {
            let other = self.ordering[j];
            if let Some(other) = self.objects.get(&other)
                && aabb.overlaps(&other.aabb())
            {
                target = j;
                break;
            }
        }
        let id = self.ordering.remove(index);
        self.ordering.insert(target, id);
    }
    fn send_forward(&mut self, id: u32) {
        let Some(index) = self.ordering.iter().position(|&x| x == id) else {
            return;
        };
        if index + 1 == self.ordering.len() {
            return;
        }
        let Some(obj) = self.objects.get(&id) else {
            return;
        };
        let aabb = obj.aabb();
        let mut target = index + 1;
        for j in (index + 1)..self.ordering.len() {
            let other = self.ordering[j];
            if let Some(other) = self.objects.get(&other)
                && aabb.overlaps(&other.aabb())
            {
                target = j;
                break;
            }
        }
        let id = self.ordering.remove(index);
        self.ordering.insert(target, id);
    }

    fn push_snapshot(
        ordering: &[u32],
        objects: &HashMap<u32, Object>,
        selected_ids: &HashSet<u32>,
        undo_snapshots: &mut Vec<Snapshot>,
        redo_snapshots: &mut Vec<Snapshot>,
    ) {
        redo_snapshots.clear();
        undo_snapshots.push(Snapshot {
            ordering: ordering.to_owned(),
            objects: objects.clone(),
            selected_ids: selected_ids.clone(),
        })
    }

    fn undo(&mut self) {
        if let Some(snapshot) = self.undo_snapshots.pop() {
            self.redo_snapshots.push(Snapshot {
                objects: self.objects.clone(),
                selected_ids: self.selected_ids.clone(),
                ordering: self.ordering.clone(),
            });
            self.objects = snapshot.objects;
            self.selected_ids = snapshot.selected_ids;
            self.ordering = snapshot.ordering;
        }
    }

    fn redo(&mut self) {
        if let Some(snapshot) = self.redo_snapshots.pop() {
            self.undo_snapshots.push(Snapshot {
                objects: self.objects.clone(),
                selected_ids: self.selected_ids.clone(),
                ordering: self.ordering.clone(),
            });
            self.objects = snapshot.objects;
            self.selected_ids = snapshot.selected_ids;
            self.ordering = snapshot.ordering;
        }
    }

    fn undo_redo(&mut self) {
        let undo = (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::LeftControl))
            && !(is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::LeftShift))
            && is_key_pressed(KeyCode::Z);
        let redo = (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::LeftControl))
            && (((is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::LeftShift))
                && is_key_pressed(KeyCode::Z))
                || is_key_pressed(KeyCode::Y));

        if undo {
            self.undo();
        }
        if redo {
            self.redo();
        }
    }

    fn state_update(&mut self) {
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
        let group_rect = get_group_rect(&self.objects, &self.selected_ids);
        match &self.state {
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
                } else if mouse_pressed {
                    let mut clicked_handle = None;
                    if !self.selected_ids.is_empty()
                        && let Some(g_rect) = group_rect
                    {
                        clicked_handle = get_active_handle(
                            g_rect,
                            mouse_pos_world,
                            HANDLE_SIZE / self.camera_zoom,
                        );
                    }

                    if let Some(handle) = clicked_handle {
                        let g_rect = group_rect.unwrap();
                        let mut initial_object_rects = HashMap::new();
                        for &id in &self.selected_ids {
                            if let Some(obj) = self.objects.get(&id) {
                                initial_object_rects.insert(id, obj.aabb());
                            }
                        }
                        Self::push_snapshot(
                            &self.ordering,
                            &self.objects,
                            &self.selected_ids,
                            &mut self.undo_snapshots,
                            &mut self.redo_snapshots,
                        );
                        self.state = InteractionState::Resizing {
                            handle,
                            initial_group_rect: g_rect,
                            initial_object_rects,
                            mouse_start: mouse_pos_world,
                        };
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
                            self.state = InteractionState::Selected {
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
            InteractionState::Selected {
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
                    for (&id, &init_pos) in initial_positions {
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
                        Object::Rect { rect, kind: *kind }.draw();
                    }
                }
            }
        }
        set_default_camera();
    }

    fn render(&mut self) {
        let raw_mouse_pos = {
            let pos = mouse_position();
            Vec2::new(pos.0, pos.1)
        };
        let camera = self.camera();
        set_camera(&camera);
        let mouse_pos_world = camera.screen_to_world(raw_mouse_pos);
        let mouse_down = is_mouse_button_down(MouseButton::Left);
        let group_rect = get_group_rect(&self.objects, &self.selected_ids);

        for id in &self.ordering {
            let obj = &self.objects[id];
            obj.draw();
        }

        if let InteractionState::AddingRectDragging { kind, start } = &self.state {
            let rect = Rect::new(
                start.x.min(mouse_pos_world.x),
                start.y.min(mouse_pos_world.y),
                (start.x - mouse_pos_world.x).abs(),
                (start.y - mouse_pos_world.y).abs(),
            );

            let show = rect.w > 2.0 && rect.h > 2.0;

            if mouse_down && show {
                Object::Rect { rect, kind: *kind }.draw();
            }
        }

        if let Some(g_rect) = group_rect {
            draw_rectangle_lines_ex(
                g_rect.x,
                g_rect.y,
                g_rect.w,
                g_rect.h,
                2.0 / self.camera_zoom,
                DrawRectangleParams {
                    color: DARKGRAY,
                    ..Default::default()
                },
            );

            for (_handle, rect) in get_handle_rects(g_rect, HANDLE_SIZE / self.camera_zoom) {
                let color = if on_handle(rect, mouse_pos_world) {
                    RED
                } else {
                    WHITE
                };
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
                draw_rectangle_lines_ex(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    1. / self.camera_zoom,
                    DrawRectangleParams {
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
        }

        if let InteractionState::Selecting { start } = &self.state {
            assert!(mouse_down);
            let x = start.x.min(mouse_pos_world.x);
            let y = start.y.min(mouse_pos_world.y);
            let w = (start.x - mouse_pos_world.x).abs();
            let h = (start.y - mouse_pos_world.y).abs();
            draw_rectangle(x, y, w, h, Color::new(0.0, 0.4, 1.0, 0.15));
            draw_rectangle_lines_ex(
                x,
                y,
                w,
                h,
                1.0,
                DrawRectangleParams {
                    color: BLUE,
                    ..Default::default()
                },
            );
        }
        set_default_camera();
        const FONT_SIZE: f32 = 20.;
        const OFFSET: f32 = 16.;
        if let Some(g_rect) = group_rect {
            draw_text(
                format!("X: {:.2}, Y: {:.2}", g_rect.x, g_rect.y),
                SIDEBAR_SIZE + OFFSET,
                FONT_SIZE / 2. + OFFSET,
                FONT_SIZE,
                DARKGRAY,
            );
            draw_text(
                format!("Size: {:.2} x {:.2}", g_rect.w, g_rect.h),
                SIDEBAR_SIZE + OFFSET,
                FONT_SIZE / 2. + OFFSET + FONT_SIZE,
                FONT_SIZE,
                DARKGRAY,
            );
        }
    }

    fn side_panel(&mut self) {
        let num = if is_key_pressed(KeyCode::Key1) {
            Some(0)
        } else if is_key_pressed(KeyCode::Key2) {
            Some(1)
        } else if is_key_pressed(KeyCode::Key3) {
            Some(2)
        } else if is_key_pressed(KeyCode::Key4) {
            Some(3)
        } else {
            None
        };
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        let raw_mouse_pos = {
            let pos = mouse_position();
            Vec2::new(pos.0, pos.1)
        };
        let buttons = [
            (RectKind::Regular, Color::from_rgba(165, 216, 255, 255)),
            (RectKind::AntiTp, Color::new(1., 0.35, 0.9, 1.)),
            (RectKind::Tar, Color::new(0.1, 0.1, 0.1, 1.0)),
            (RectKind::Hot, Color::new(1.0, 0.8, 0.15, 1.0)),
        ];

        if let Some(num) = num {
            if let InteractionState::AddingRectSelected { kind } = &self.state {
                if buttons[num].0 == *kind {
                    self.state = InteractionState::None;
                }
            } else {
                self.state = InteractionState::AddingRectSelected {
                    kind: buttons[num].0,
                }
            }
        }

        let mut start = 0.;
        draw_rectangle(0., 0., SIDEBAR_SIZE, screen_height(), WHITE);

        for (kind, color) in buttons {
            let rect = Rect::new(
                (SIDEBAR_SIZE - PATCH_SIZE) / 2.,
                start + PATCH_SIZE / 2.,
                PATCH_SIZE,
                PATCH_SIZE,
            );
            let outer_rect = Rect::new(0., start, SIDEBAR_SIZE, SIDEBAR_SIZE);
            start += SIDEBAR_SIZE;
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
            if let InteractionState::AddingRectSelected {
                kind: interact_rectkind,
            }
            | InteractionState::AddingRectDragging {
                kind: interact_rectkind,
                ..
            } = &self.state
                && kind == *interact_rectkind
            {
                draw_rectangle_lines_ex(
                    rect.x - 2.,
                    rect.y - 2.,
                    rect.w + 4.,
                    rect.h + 4.,
                    3.,
                    DrawRectangleParams {
                        color: GRAY,
                        ..Default::default()
                    },
                )
            }

            if mouse_pressed && outer_rect.contains(raw_mouse_pos) {
                self.state = InteractionState::AddingRectSelected { kind };
            }
        }
    }

    fn context_menu(&mut self) {
        let Some(ContextMenu { pos, target }) = &mut self.context_menu else {
            return;
        };

        let items = target.options();
        const ITEM_HEIGHT: f32 = 50.;
        let contextmenu_size: Vec2 = Vec2::new(140., ITEM_HEIGHT * items.len() as f32);
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

#[macroquad::main("Level design")]
async fn main() {
    let mut app = App::new();
    loop {
        clear_background(LIGHTGRAY);
        let raw_mouse_pos = {
            let pos = mouse_position();
            Vec2::new(pos.0, pos.1)
        };
        let in_sidebar = raw_mouse_pos.x <= SIDEBAR_SIZE;

        app.camera_zoom();
        app.camera_pan();
        app.undo_redo();
        if !in_sidebar {
            app.state_update();
        }
        app.render();
        app.side_panel();

        app.context_menu();

        next_frame().await;
    }
}

fn on_handle(handle: Rect, mouse: Vec2) -> bool {
    let extra = 10.;
    let half = extra / 2.;
    let rect = Rect {
        x: handle.x - half,
        y: handle.y - half,
        h: handle.h + extra,
        w: handle.w + extra,
    };
    rect.contains(mouse)
}

fn get_group_rect(objects: &HashMap<u32, Object>, selected_ids: &HashSet<u32>) -> Option<Rect> {
    if selected_ids.is_empty() {
        return None;
    }

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for id in selected_ids {
        if let Some(obj) = objects.get(id) {
            let size = obj.size();
            let pos = obj.pos();
            min_x = min_x.min(pos.x);
            min_y = min_y.min(pos.y);
            max_x = max_x.max(pos.x + size.x);
            max_y = max_y.max(pos.y + size.y);
        }
    }

    Some(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
}

fn get_handle_rects(r: Rect, size: f32) -> [(HandleKind, Rect); 8] {
    let half = size / 2.0;
    let cx = r.x + r.w / 2.0;
    let cy = r.y + r.h / 2.0;

    [
        (
            HandleKind::TopLeft,
            Rect::new(r.x - half, r.y - half, size, size),
        ),
        (
            HandleKind::TopMiddle,
            Rect::new(cx - half, r.y - half, size, size),
        ),
        (
            HandleKind::TopRight,
            Rect::new(r.x + r.w - half, r.y - half, size, size),
        ),
        (
            HandleKind::RightMiddle,
            Rect::new(r.x + r.w - half, cy - half, size, size),
        ),
        (
            HandleKind::BottomRight,
            Rect::new(r.x + r.w - half, r.y + r.h - half, size, size),
        ),
        (
            HandleKind::BottomMiddle,
            Rect::new(cx - half, r.y + r.h - half, size, size),
        ),
        (
            HandleKind::BottomLeft,
            Rect::new(r.x - half, r.y + r.h - half, size, size),
        ),
        (
            HandleKind::LeftMiddle,
            Rect::new(r.x - half, cy - half, size, size),
        ),
    ]
}

fn get_active_handle(r: Rect, mouse: Vec2, size: f32) -> Option<HandleKind> {
    for (handle, rect) in get_handle_rects(r, size) {
        if on_handle(rect, mouse) {
            return Some(handle);
        }
    }
    None
}

fn resized(start: Rect, handle_kind: HandleKind, delta: Vec2) -> Rect {
    let mut x = start.x;
    let mut y = start.y;
    let mut w = start.w;
    let mut h = start.h;

    let min_size = 10.0;

    match handle_kind {
        HandleKind::TopLeft => {
            x += delta.x;
            y += delta.y;
            w -= delta.x;
            h -= delta.y;
        }
        HandleKind::TopMiddle => {
            y += delta.y;
            h -= delta.y;
        }
        HandleKind::TopRight => {
            y += delta.y;
            w += delta.x;
            h -= delta.y;
        }
        HandleKind::RightMiddle => {
            w += delta.x;
        }
        HandleKind::BottomRight => {
            w += delta.x;
            h += delta.y;
        }
        HandleKind::BottomMiddle => {
            h += delta.y;
        }
        HandleKind::BottomLeft => {
            x += delta.x;
            w -= delta.x;
            h += delta.y;
        }
        HandleKind::LeftMiddle => {
            x += delta.x;
            w -= delta.x;
        }
    }

    if w < min_size {
        if matches!(
            handle_kind,
            HandleKind::TopLeft | HandleKind::BottomLeft | HandleKind::LeftMiddle
        ) {
            x = start.x + start.w - min_size;
        }
        w = min_size;
    }
    if h < min_size {
        if matches!(
            handle_kind,
            HandleKind::TopLeft | HandleKind::TopMiddle | HandleKind::TopRight
        ) {
            y = start.y + start.h - min_size;
        }
        h = min_size;
    }

    Rect::new(x, y, w, h)
}

pub fn sat(rect_a: Rect, rot_a: f32, rect_b: Rect, rot_b: f32) -> bool {
    fn obb(rect: Rect, rot: f32) -> (Vec2, Vec2, Vec2, Vec2) {
        let center = Vec2::new(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        let half = Vec2::new(rect.w * 0.5, rect.h * 0.5);
        let (c, s) = (rot.cos(), rot.sin());
        let x = Vec2::new(c, s);
        let y = Vec2::new(-s, c);
        (center, half, x, y)
    }

    let (center_a, half_a, ax, ay) = obb(rect_a, rot_a);
    let (center_b, half_b, bx, by) = obb(rect_b, rot_b);

    let trans = center_b - center_a;
    let axes = [ax, ay, bx, by];

    for axis in axes {
        let projection_dist = (trans.dot(axis)).abs();
        let r_a = half_a.x * (ax.dot(axis)).abs() + half_a.y * (ay.dot(axis)).abs();
        let r_b = half_b.x * (bx.dot(axis)).abs() + half_b.y * (by.dot(axis)).abs();

        if projection_dist > r_a + r_b {
            return false;
        }
    }

    true
}
