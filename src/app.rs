use super::*;
#[derive(Debug, Clone)]
pub struct Textures {
    pub bird: Texture2D,
    pub purple_bird: Texture2D,
    pub laser: Texture2D,
    pub banana: Texture2D,
    pub strawberry: Texture2D,
    pub heart: Texture2D,
    pub player: Texture2D,
    pub checkpoint: Texture2D,
    pub wave: Texture2D,
    pub tutorial_textures: HashMap<&'static str, Texture2D>,
    pub green: Texture2D,
}
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub ordering: Vec<u32>,
    pub objects: HashMap<u32, Object>,
    pub selected_ids: HashSet<u32>,
}

#[derive(Debug, Clone)]
pub struct App {
    pub state: InteractionState,
    pub ordering: Vec<u32>,
    pub objects: HashMap<u32, Object>,
    pub selected_ids: HashSet<u32>,
    pub undo_snapshots: Vec<Snapshot>,
    pub redo_snapshots: Vec<Snapshot>,
    pub context_menu: Option<ContextMenu>,
    pub camera_zoom: f32,
    pub camera_pos: Vec2,
    pub pan_start: Vec2,
    pub textures: Textures,
    pub clipboard: Vec<Object>,
}

impl App {
    pub fn copy(&self) -> Vec<Object> {
        let mut v = Vec::new();
        for id in self.ordering.iter() {
            if !self.selected_ids.contains(id) {
                continue;
            }
            v.push(self.objects[id]);
        }
        v
    }
    pub fn paste(&mut self, pos: Vec2) {
        if self.clipboard.is_empty() {
            return;
        }
        Self::push_snapshot(
            &self.ordering,
            &self.objects,
            &self.selected_ids,
            &mut self.undo_snapshots,
            &mut self.redo_snapshots,
        );
        let v = pos - self.clipboard[0].pos();
        self.selected_ids.clear();
        for mut obj in self.clipboard.iter().copied() {
            let id = new_id();
            obj.set_pos(obj.pos() + v);
            self.selected_ids.insert(id);
            self.objects.insert(id, obj);
            self.ordering.push(id);
        }
    }
    pub fn dupe(&mut self) {
        let mut copied = self.copy();
        if copied.is_empty() {
            return;
        }
        let pos = copied[0].pos() + Vec2::new(10., 10.);
        std::mem::swap(&mut copied, &mut self.clipboard);
        self.paste(pos);
        std::mem::swap(&mut copied, &mut self.clipboard);
    }
    pub async fn new() -> Self {
        let bird = load_texture("textures/bird.png").await.unwrap();
        let purple_bird = load_texture("textures/purple-bird.png").await.unwrap();
        let banana = load_texture("textures/bananas-pink.png").await.unwrap();
        let strawberry = load_texture("textures/strawberry-orange.png")
            .await
            .unwrap();
        let heart = load_texture("textures/heart-pink.png").await.unwrap();
        let laser = load_texture("textures/laser.png").await.unwrap();
        let player = load_texture("textures/player.png").await.unwrap();
        let checkpoint = load_texture("textures/checkpoint.png").await.unwrap();
        let wave = load_texture("textures/wave.png").await.unwrap();

        let map = HashMap::from([
            (
                "checkpoint",
                load_texture("textures/tutorial/tutorial-checkpoint.png")
                    .await
                    .unwrap(),
            ),
            (
                "close_window",
                load_texture("textures/tutorial/tutorial-close-window.png")
                    .await
                    .unwrap(),
            ),
            (
                "bird",
                load_texture("textures/tutorial/tutorial-bird.png")
                    .await
                    .unwrap(),
            ),
            (
                "laser",
                load_texture("textures/tutorial/tutorial-laser.png")
                    .await
                    .unwrap(),
            ),
            (
                "wave",
                load_texture("textures/tutorial/tutorial-wave.png")
                    .await
                    .unwrap(),
            ),
            (
                "framerate",
                load_texture("textures/tutorial/tutorial-framerate.png")
                    .await
                    .unwrap(),
            ),
            (
                "fruits",
                load_texture("textures/tutorial/tutorial-fruits.png")
                    .await
                    .unwrap(),
            ),
            (
                "heart",
                load_texture("textures/tutorial/tutorial-heart.png")
                    .await
                    .unwrap(),
            ),
            (
                "jump",
                load_texture("textures/tutorial/tutorial-jump.png")
                    .await
                    .unwrap(),
            ),
            (
                "move",
                load_texture("textures/tutorial/tutorial-move.png")
                    .await
                    .unwrap(),
            ),
            (
                "reset",
                load_texture("textures/tutorial/tutorial-reset.png")
                    .await
                    .unwrap(),
            ),
            (
                "hot",
                load_texture("textures/tutorial/tutorial-surface-hot.png")
                    .await
                    .unwrap(),
            ),
            (
                "pink",
                load_texture("textures/tutorial/tutorial-surface-pink.png")
                    .await
                    .unwrap(),
            ),
            (
                "tar1",
                load_texture("textures/tutorial/tutorial-surface-tar-1.png")
                    .await
                    .unwrap(),
            ),
            (
                "tar2",
                load_texture("textures/tutorial/tutorial-surface-tar-2.png")
                    .await
                    .unwrap(),
            ),
            (
                "teleport",
                load_texture("textures/tutorial/tutorial-teleport.png")
                    .await
                    .unwrap(),
            ),
            (
                "walljump",
                load_texture("textures/tutorial/tutorial-walljump.png")
                    .await
                    .unwrap(),
            ),
            (
                "congrats",
                load_texture("textures/tutorial/congratulations-screen.png")
                    .await
                    .unwrap(),
            ),
            (
                "portal",
                load_texture("textures/tutorial/portal-purple.png")
                    .await
                    .unwrap(),
            ),
        ]);

        let green_img = Image::gen_image_color(100, 100, Color::from_rgba(135, 255, 139, 255));
        let green = Texture2D::from_image(&green_img);

        let textures = Textures {
            bird,
            banana,
            strawberry,
            heart,
            laser,
            player,
            tutorial_textures: map,
            checkpoint,
            wave,
            green,
            purple_bird,
        };
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
            textures,
            clipboard: Vec::new(),
        }
    }

    pub fn push_object(&mut self, object: Object) {
        let id = new_id();
        self.objects.insert(id, object);
        self.ordering.push(id);
    }

    pub fn camera(&self) -> Camera2D {
        Camera2D {
            zoom: self.camera_zoom * Vec2::new(2.0 / screen_width(), -2.0 / screen_height()),
            target: self.camera_pos,
            ..Default::default()
        }
    }

    pub fn camera_zoom(&mut self) {
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

    pub fn camera_pan(&mut self) -> bool {
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

    pub fn send_to_back_single(&mut self, id: u32) {
        if let Some(pos) = self.ordering.iter().position(|&x| x == id) {
            let id = self.ordering.remove(pos);
            self.ordering.insert(0, id);
        }
    }
    pub fn send_to_front_single(&mut self, id: u32) {
        if let Some(pos) = self.ordering.iter().position(|&x| x == id) {
            let id = self.ordering.remove(pos);
            self.ordering.push(id);
        }
    }

    pub fn send_to_back_multi(&mut self) {
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

    pub fn send_to_front_multi(&mut self) {
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

    pub fn send_backward(&mut self, id: u32) {
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
    pub fn send_forward(&mut self, id: u32) {
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

    pub fn push_snapshot(
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

    pub fn undo_redo(&mut self) {
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

    pub fn render(&mut self) {
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
            obj.draw(&self.textures);
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
                Object::Rect { rect, kind: *kind }.draw(&self.textures);
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
                    color: Color::from_rgba(100, 50, 255, 255),
                    ..Default::default()
                },
            );

            if self.selected_ids.len() == 1 {
                let id = self.selected_ids.iter().next().unwrap();
                let object = &self.objects[id];
                draw_handles(
                    object,
                    mouse_pos_world,
                    HANDLE_SIZE / self.camera_zoom,
                    self.camera_zoom,
                );
            } else {
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
        }

        if let InteractionState::Selecting { start } = &self.state {
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
}

pub fn draw_cone(mid: Vec2, radius: f32, angle: f32, width: f32, color: Color) {
    let half_width = width / 2.0;
    let start = angle - half_width;
    let segments = ((width.to_degrees() as u32) / 10).max(5);
    let step = width / segments as f32;
    let mut prev_point = mid + Vec2::from_angle(start) * radius;
    for i in 1..=segments {
        let curr_angle = start + step * i as f32;
        let curr_point = mid + Vec2::from_angle(curr_angle) * radius;
        draw_triangle(mid, prev_point, curr_point, color);
        prev_point = curr_point;
    }
}

pub fn get_group_rect(objects: &HashMap<u32, Object>, selected_ids: &HashSet<u32>) -> Option<Rect> {
    if selected_ids.is_empty() {
        return None;
    }

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for id in selected_ids {
        if let Some(obj) = objects.get(id) {
            let rect = obj.grect_aabb();
            let size = Vec2::new(rect.w, rect.h);
            let pos = Vec2::new(rect.x, rect.y);
            min_x = min_x.min(pos.x);
            min_y = min_y.min(pos.y);
            max_x = max_x.max(pos.x + size.x);
            max_y = max_y.max(pos.y + size.y);
        }
    }

    Some(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
}
