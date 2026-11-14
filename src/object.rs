use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RectKind {
    Regular,
    AntiTp,
    Tar,
    Hot,
    PathfindingBound,
}

#[derive(Clone, Copy, Debug)]
pub enum Object {
    Rect {
        rect: Rect,
        kind: RectKind,
    },
    Laser {
        rail_start: Vec2,
        rail_end: Vec2,
        rel: f32,
    },
    Player {
        pos: Vec2,
    },
    Bird {
        rect: Rect,
        detection_range: f32,
    },
    Heart {
        rect: Rect,
    },
    Banana {
        rect: Rect,
    },
    WaypointMarker {
        rect: Rect,
        waypoint_id: i32,
    },
    Strawberry {
        rect: Rect,
    },
    Checkpoint {
        rect: Rect,
    },
    Wave {
        rect: Rect,
        radius: f32,
        angle: f32,
        width: f32,
    },
    TutorialTexture {
        id: &'static str,
        rect: Rect,
    },
}

impl Object {
    pub fn same_type(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Object::Rect { .. }, Object::Rect { .. })
                | (Object::Laser { .. }, Object::Laser { .. })
                | (Object::Player { .. }, Object::Player { .. })
                | (Object::Bird { .. }, Object::Bird { .. })
                | (Object::Heart { .. }, Object::Heart { .. })
                | (Object::Banana { .. }, Object::Banana { .. })
                | (Object::Strawberry { .. }, Object::Strawberry { .. })
                | (Object::Checkpoint { .. }, Object::Checkpoint { .. })
                | (Object::Wave { .. }, Object::Wave { .. })
                | (
                    Object::TutorialTexture { .. },
                    Object::TutorialTexture { .. }
                )
                | (Object::WaypointMarker { .. }, Object::WaypointMarker { .. })
        )
    }
    pub fn aabb(&self) -> Rect {
        match self {
            Object::Rect { rect, .. } => *rect,
            Object::Laser {
                rail_start,
                rail_end,
                rel,
            } => {
                let pos = rail_start.lerp(*rail_end, *rel);
                Rect::new(
                    pos.x - LASER_WIDTH / 2.,
                    pos.y - LASER_WIDTH / 2.,
                    LASER_WIDTH,
                    LASER_WIDTH,
                )
            }

            Object::Player { pos } => Rect::new(pos.x, pos.y, PLAYER_WIDTH, PLAYER_HEIGHT),
            Object::WaypointMarker { rect, .. }
            | Object::Checkpoint { rect }
            | Object::Wave { rect, .. }
            | Object::TutorialTexture { rect, .. }
            | Object::Bird { rect, .. }
            | Object::Heart { rect }
            | Object::Banana { rect }
            | Object::Strawberry { rect } => *rect,
        }
    }
    pub fn grect_aabb(&self) -> Rect {
        if let Object::Laser {
            rail_start,
            rail_end,
            ..
        } = self
        {
            let min = rail_start.min(*rail_end);
            let max = rail_start.max(*rail_end);
            let size = max - min;
            let pos = min;

            Rect::new(pos.x, pos.y, size.x, size.y).combine_with(self.aabb())
        } else {
            self.aabb()
        }
    }
    pub fn contains(&self, pos: Vec2) -> bool {
        self.aabb().contains(pos)
    }
    pub fn pos(&self) -> Vec2 {
        self.aabb().point()
    }
    pub fn set_pos(&mut self, pos: Vec2) {
        match self {
            Object::Rect { rect, .. } => {
                rect.x = pos.x;
                rect.y = pos.y;
            }
            Object::Laser {
                rail_start,
                rail_end,
                rel,
            } => {
                let old_pos = rail_start.lerp(*rail_end, *rel) - LASER_WIDTH / 2.;
                let delta = pos - old_pos;
                *rail_start += delta;
                *rail_end += delta;
            }
            Object::Player { pos: p } => *p = pos,
            Object::WaypointMarker { rect, .. }
            | Object::Checkpoint { rect }
            | Object::Wave { rect, .. }
            | Object::TutorialTexture { rect, .. }
            | Object::Bird { rect, .. }
            | Object::Heart { rect }
            | Object::Banana { rect }
            | Object::Strawberry { rect } => {
                rect.x = pos.x;
                rect.y = pos.y;
            }
        }
    }
    pub fn set_pos_centered(&mut self, pos: Vec2) {
        match self {
            Object::Rect { rect, .. } => {
                rect.x = pos.x - rect.w / 2.;
                rect.y = pos.y - rect.h / 2.;
            }
            Object::WaypointMarker { rect, .. } => {
                rect.x = pos.x - rect.w / 2.;
                rect.y = pos.y - rect.h / 2.;
            }
            Object::Laser {
                rail_start,
                rail_end,
                rel,
            } => {
                let old_pos = rail_start.lerp(*rail_end, *rel) - LASER_WIDTH / 2.;
                let delta = pos - old_pos;
                *rail_start += delta;
                *rail_end += delta;
            }
            Object::Player { pos: p } => {
                p.x = pos.x - PLAYER_WIDTH / 2.;
                p.y = pos.y - PLAYER_HEIGHT / 2.;
            }
            Object::Checkpoint { rect }
            | Object::Wave { rect, .. }
            | Object::TutorialTexture { rect, .. }
            | Object::Bird { rect, .. }
            | Object::Heart { rect }
            | Object::Banana { rect }
            | Object::Strawberry { rect } => {
                rect.x = pos.x - rect.w / 2.;
                rect.y = pos.y - rect.h / 2.;
            }
        }
    }
    pub fn set_size(&mut self, size: Vec2) {
        match self {
            Object::Rect { rect, .. } => {
                rect.w = size.x;
                rect.h = size.y;
            }
            Object::WaypointMarker { .. }
            | Object::Checkpoint { .. }
            | Object::Wave { .. }
            | Object::TutorialTexture { .. }
            | Object::Bird { .. }
            | Object::Heart { .. }
            | Object::Banana { .. }
            | Object::Strawberry { .. } => {
                // rect.w = size.x;
                // rect.h = size.y;
            }
            Object::Player { .. } | Object::Laser { .. } => {}
        }
    }
    pub fn overlaps_rect(&self, rect: Rect) -> bool {
        self.aabb().overlaps(&rect)
    }
    pub fn draw(&self, textures: &Textures) {
        self.draw_with_alpha(textures, 1.0);
    }
    pub fn draw_with_alpha(&self, textures: &Textures, alpha: f32) {
        let alpha_u8 = (alpha * 255.) as u8;
        match self {
            Object::Rect { rect, kind } => {
                let (fill, outline) = match kind {
                    RectKind::Regular => (
                        Color::from_rgba(165, 216, 255, alpha_u8),
                        Color::new(0.5, 0.5, 0.5, alpha),
                    ),
                    RectKind::AntiTp => (
                        Color::new(1., 0.35, 0.9, alpha),
                        Color::new(0.65, 0.2, 0.5, alpha),
                    ),
                    RectKind::Tar => (
                        Color::new(0.1, 0.1, 0.1, alpha),
                        Color::new(0.5, 0.5, 0.5, alpha),
                    ),
                    RectKind::Hot => (
                        Color::new(1.0, 0.8, 0.15, alpha),
                        Color::new(1.0, 0.7, 0.15, alpha),
                    ),
                    RectKind::PathfindingBound => (
                        Color::new(0., 1., 0.85, 0.1 * alpha),
                        Color::new(0., 0.58, 0.58, 0.1 * alpha),
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
            Object::WaypointMarker { rect, waypoint_id } => {
                let s = waypoint_id.to_string();
                let font_size = 100;
                let font_scale = 0.25;
                let size = measure_text(&s, None, font_size, font_scale);
                let pos = Vec2::new(
                    rect.x + rect.w / 2. - size.width / 2.,
                    rect.y + rect.h / 2. - size.height / 2.,
                );
                draw_rectangle_ex(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    DrawRectangleParams {
                        color: Color::from_rgba(135, 255, 139, alpha_u8),
                        ..Default::default()
                    },
                );
                draw_text_ex(
                    s,
                    pos.x,
                    pos.y,
                    TextParams {
                        font_size,
                        font_scale: -font_scale,
                        font_scale_aspect: -1.,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
            Object::Laser {
                rail_start,
                rail_end,
                rel,
            } => {
                draw_line(
                    rail_start.x,
                    rail_start.y,
                    rail_end.x,
                    rail_end.y,
                    2.,
                    Color::from_rgba(15, 25, 40, alpha_u8),
                );
                let dir = *rail_end - *rail_start;
                let rot = dir.y.atan2(dir.x);
                let pos = rail_start.lerp(*rail_end, *rel);
                let perp = -dir.perp().normalize_or_zero();
                let len = 200.;
                let end = pos + perp * len;
                draw_line(pos.x, pos.y, end.x, end.y, 5., RED.with_alpha(alpha));
                draw_rectangle_ex(
                    pos.x,
                    pos.y,
                    LASER_WIDTH,
                    LASER_HEIGHT,
                    DrawRectangleParams {
                        offset: Vec2::new(0.5, 0.5),
                        rotation: rot,
                        color: BLACK.with_alpha(alpha),
                    },
                );
            }
            Object::Player { pos } => {
                // let x = pos.x;
                // let y = pos.y;
                // let w = PLAYER_WIDTH;
                // let h = PLAYER_HEIGHT;

                // draw_rectangle(x, y, w, h, BLUE.with_alpha(alpha));

                // let eye = Vec2::splat(6.0);

                // let y = y + h * 0.25;
                // let lx = x + w * 0.25 - eye.x * 0.5;
                // let rx = x + w * 0.75 - eye.x * 0.5;

                // draw_rectangle(lx, y, eye.x, eye.y, WHITE.with_alpha(alpha));
                // draw_rectangle(rx, y, eye.x, eye.y, WHITE.with_alpha(alpha));
                draw_texture_ex(
                    &textures.player,
                    pos.x,
                    pos.y,
                    WHITE.with_alpha(alpha),
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                        flip_y: true,
                        ..Default::default()
                    },
                )
            }
            Object::Bird {
                rect,
                detection_range,
            } => {
                let pos = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                draw_circle(
                    pos.x,
                    pos.y,
                    *detection_range,
                    Color::from_rgba(50, 168, 153, (50_f32 * alpha) as u8),
                );
                draw_texture_ex(
                    &textures.bird,
                    rect.x,
                    rect.y,
                    WHITE.with_alpha(alpha),
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(rect.w, rect.h)),
                        flip_y: true,
                        ..Default::default()
                    },
                );
            }
            Object::Heart { rect } => {
                draw_texture_ex(
                    &textures.heart,
                    rect.x,
                    rect.y,
                    WHITE.with_alpha(alpha),
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(rect.w, rect.h)),
                        flip_y: true,
                        ..Default::default()
                    },
                );
            }
            Object::Banana { rect } => {
                draw_texture_ex(
                    &textures.banana,
                    rect.x,
                    rect.y,
                    WHITE.with_alpha(alpha),
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(rect.w, rect.h)),
                        flip_y: true,
                        ..Default::default()
                    },
                );
            }
            Object::Strawberry { rect } => {
                draw_texture_ex(
                    &textures.strawberry,
                    rect.x,
                    rect.y,
                    WHITE.with_alpha(alpha),
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(rect.w, rect.h)),
                        flip_y: true,
                        ..Default::default()
                    },
                );
            }
            Object::TutorialTexture { id, rect } => {
                let tex = &textures.tutorial_textures[id];
                draw_texture_ex(
                    tex,
                    rect.x,
                    rect.y,
                    WHITE.with_alpha(alpha),
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(rect.w, rect.h)),
                        flip_y: true,
                        ..Default::default()
                    },
                );
            }
            Object::Checkpoint { rect } => {
                draw_texture_ex(
                    &textures.checkpoint,
                    // rect.x + rect.w,
                    rect.x,
                    rect.y,
                    WHITE.with_alpha(alpha),
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(rect.w, rect.h)),
                        flip_y: true,
                        ..Default::default()
                    },
                );
            }
            Object::Wave {
                rect,
                radius,
                angle,
                width,
            } => {
                let pos = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                // draw_circle(
                //     pos.x,
                //     pos.y,
                //     *radius,
                //     Color::from_rgba(50, 168, 153, (50_f32 * alpha) as u8),
                // );
                draw_cone(
                    pos,
                    *radius,
                    *angle,
                    *width,
                    Color::from_rgba(50, 168, 153, (50_f32 * alpha) as u8),
                );
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, BLACK);
            }
        }
    }

    pub fn context_menu_target(&self, id: u32) -> ContextMenuTarget {
        if let Object::WaypointMarker { .. } = self {
            return ContextMenuTarget::Waypoint {
                id,
                waypoint_id: String::new(),
            };
        }
        ContextMenuTarget::SingleObject { id }
    }
}
