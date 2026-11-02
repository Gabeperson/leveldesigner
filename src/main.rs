use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone, Copy, Debug)]
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
}

#[derive(Clone, Copy)]
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

#[derive(Clone)]
enum InteractionState {
    None,
    Selecting {
        start: Vec2,
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
}

const HANDLE_SIZE: f32 = 10.0;

fn new_id() -> u32 {
    static ID: AtomicU32 = AtomicU32::new(1);
    ID.fetch_add(1, Ordering::Relaxed)
}

#[macroquad::main("Level design - Multi-Select")]
async fn main() {
    let mut objects = HashMap::new();

    let id1 = new_id();
    objects.insert(
        id1,
        Object::Rect {
            rect: Rect::new(200.0, 150.0, 180.0, 120.0),
            kind: RectKind::Regular,
        },
    );

    let id2 = new_id();
    objects.insert(
        id2,
        Object::Rect {
            rect: Rect::new(420.0, 220.0, 150.0, 100.0),
            kind: RectKind::Regular,
        },
    );

    let mut selected_ids: HashSet<u32> = HashSet::new();
    let mut state = InteractionState::None;

    loop {
        clear_background(LIGHTGRAY);

        let mouse_pos = {
            let pos = mouse_position();
            Vec2::new(pos.0, pos.1)
        };
        let mouse_down = is_mouse_button_down(MouseButton::Left);
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        let shift_down = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);

        let group_rect = get_group_rect(&objects, &selected_ids);

        match &state {
            InteractionState::None => {
                if mouse_pressed {
                    let mut clicked_handle = None;
                    if !selected_ids.is_empty()
                        && let Some(g_rect) = group_rect
                    {
                        clicked_handle = get_active_handle(g_rect, mouse_pos, HANDLE_SIZE);
                    }

                    if let Some(handle) = clicked_handle {
                        let g_rect = group_rect.unwrap();
                        let mut initial_object_rects = HashMap::new();
                        for &id in &selected_ids {
                            if let Some(obj) = objects.get(&id) {
                                initial_object_rects.insert(id, obj.aabb());
                            }
                        }
                        state = InteractionState::Resizing {
                            handle,
                            initial_group_rect: g_rect,
                            initial_object_rects,
                            mouse_start: mouse_pos,
                        };
                    } else {
                        let mut clicked_id = None;
                        for (&id, obj) in &objects {
                            if obj.contains(mouse_pos) {
                                clicked_id = Some(id);
                                break;
                            }
                        }

                        if let Some(id) = clicked_id {
                            if shift_down {
                                if selected_ids.contains(&id) {
                                    selected_ids.remove(&id);
                                } else {
                                    selected_ids.insert(id);
                                }
                            } else {
                                if !selected_ids.contains(&id) {
                                    selected_ids.clear();
                                    selected_ids.insert(id);
                                }
                            }

                            let mut initial_positions = HashMap::new();
                            for &sel_id in &selected_ids {
                                if let Some(obj) = objects.get(&sel_id) {
                                    initial_positions.insert(sel_id, obj.pos());
                                }
                            }
                            state = InteractionState::Moving {
                                initial_positions,
                                mouse_start: mouse_pos,
                            };
                        } else {
                            if !shift_down {
                                selected_ids.clear();
                            }
                            state = InteractionState::Selecting { start: mouse_pos };
                        }
                    }
                }
            }
            InteractionState::Selecting { start } => {
                if mouse_down {
                    let x = start.x.min(mouse_pos.x);
                    let y = start.y.min(mouse_pos.y);
                    let w = (start.x - mouse_pos.x).abs();
                    let h = (start.y - mouse_pos.y).abs();
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
                } else {
                    let sel_rect = Rect::new(
                        start.x.min(mouse_pos.x),
                        start.y.min(mouse_pos.y),
                        (start.x - mouse_pos.x).abs(),
                        (start.y - mouse_pos.y).abs(),
                    );

                    if !shift_down {
                        selected_ids.clear();
                    }

                    for (&id, obj) in &objects {
                        if obj.overlaps_rect(sel_rect) {
                            selected_ids.insert(id);
                        }
                    }
                    state = InteractionState::None;
                }
            }
            InteractionState::Moving {
                initial_positions,
                mouse_start,
            } => {
                if mouse_down {
                    let delta = mouse_pos - *mouse_start;
                    for (&id, &init_pos) in initial_positions {
                        if let Some(obj) = objects.get_mut(&id) {
                            obj.set_pos(init_pos + delta);
                        }
                    }
                } else {
                    state = InteractionState::None;
                }
            }
            InteractionState::Resizing {
                handle,
                initial_group_rect,
                initial_object_rects,
                mouse_start,
            } => {
                if mouse_down {
                    let delta = mouse_pos - *mouse_start;
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
                        if let Some(obj) = objects.get_mut(&id) {
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
                    state = InteractionState::None;
                }
            }
        }

        for obj in objects.values() {
            obj.draw();
        }

        if let Some(g_rect) = group_rect {
            draw_rectangle_lines_ex(
                g_rect.x,
                g_rect.y,
                g_rect.w,
                g_rect.h,
                2.0,
                DrawRectangleParams {
                    color: DARKGRAY,
                    ..Default::default()
                },
            );

            for (_handle, rect) in get_handle_rects(g_rect, HANDLE_SIZE) {
                let color = if on_handle(rect, mouse_pos) {
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
                    1.,
                    DrawRectangleParams {
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
        }

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
            let rect = obj.aabb();
            min_x = min_x.min(rect.x);
            min_y = min_y.min(rect.y);
            max_x = max_x.max(rect.x + rect.w);
            max_y = max_y.max(rect.y + rect.h);
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
