use super::*;
impl App {
    pub fn side_panel(&mut self) {
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
            (
                RectKind::PathfindingBound,
                Color::from_rgba(0, 255, 217, 50),
            ),
        ];

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
        let add_items = [
            (
                &self.textures.laser,
                Object::Laser {
                    rail_start: Vec2::new(-100., 0.),
                    rail_end: Vec2::new(100., 0.),
                    rel: 0.5,
                },
            ),
            (
                &self.textures.player,
                Object::Player {
                    pos: Vec2::splat(0.0),
                },
            ),
            (
                &self.textures.bird,
                Object::Bird {
                    rect: Rect::new(0., 0., 50., 50.),
                    detection_range: 200.,
                    kind: BirdKind::Pathfinding,
                    can_leave_spawn: false,
                },
            ),
            (
                &self.textures.heart,
                Object::Heart {
                    rect: Rect::new(0., 0., 48., 48.),
                },
            ),
            (
                &self.textures.banana,
                Object::Banana {
                    rect: Rect::new(0., 0., 48., 48.),
                },
            ),
            (
                &self.textures.strawberry,
                Object::Strawberry {
                    rect: Rect::new(0., 0., 48., 48.),
                },
            ),
            (
                &self.textures.checkpoint,
                Object::Checkpoint {
                    rect: Rect::new(0.0, 0.0, 60., 60.),
                },
            ),
            (
                &self.textures.wave,
                Object::Wave {
                    rect: Rect::new(0.0, 0.0, 40., 40.),
                    radius: 200.,
                    angle: 270.0f32.to_radians(),
                    width: 90.0f32.to_radians(),
                },
            ),
            (
                &self.textures.green,
                Object::WaypointMarker {
                    rect: Rect::new(0.0, 0.0, WAYPOINT_SIZE, WAYPOINT_SIZE),
                    waypoint_id: 0,
                },
            ),
        ];
        for (texture, object) in add_items {
            let rect = Rect::new(
                (SIDEBAR_SIZE - PATCH_SIZE) / 2.,
                start + PATCH_SIZE / 2.,
                PATCH_SIZE,
                PATCH_SIZE,
            );
            let outer_rect = Rect::new(0., start, SIDEBAR_SIZE, SIDEBAR_SIZE);
            start += SIDEBAR_SIZE;
            draw_texture_ex(
                texture,
                rect.x,
                rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(rect.w, rect.h)),
                    ..Default::default()
                },
            );
            if let InteractionState::AddingObject { object: adding_obj } = &self.state
                && adding_obj.same_type(&object)
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
                self.state = InteractionState::AddingObject { object };
            }
        }
    }
}
