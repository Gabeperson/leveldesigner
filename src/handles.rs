use super::*;

#[derive(Debug, Clone, Copy)]
pub enum HandleKind {
    TopLeft,
    TopMiddle,
    TopRight,
    RightMiddle,
    BottomRight,
    BottomMiddle,
    BottomLeft,
    LeftMiddle,
    RailStart,
    RailEnd,
    MoveLaserBase,
    DetectionRange,
    ConeWidth,
    ConeAngleRadius,
}

pub fn get_handle_rects(r: Rect, size: f32) -> [(HandleKind, Rect); 8] {
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

pub fn get_active_handle_group(r: Rect, mouse: Vec2, size: f32) -> Option<HandleKind> {
    for (handle, rect) in get_handle_rects(r, size) {
        if on_handle(rect, mouse) {
            return Some(handle);
        }
    }
    None
}

pub fn get_active_handle(obj: &Object, mouse: Vec2, size: f32) -> Option<HandleKind> {
    match obj {
        Object::Rect { rect, .. } => {
            for (handle, rect) in get_handle_rects(*rect, size) {
                if on_handle(rect, mouse) {
                    return Some(handle);
                }
            }
        }
        Object::Laser {
            rail_start,
            rail_end,
            rel,
        } => {
            if Circle::new(rail_start.x, rail_start.y, size).contains(&mouse) {
                return Some(HandleKind::RailStart);
            } else if Circle::new(rail_end.x, rail_end.y, size).contains(&mouse) {
                return Some(HandleKind::RailEnd);
            }
            let pos = rail_start.lerp(*rail_end, *rel);
            let dir = *rail_end - *rail_start;
            let perp = dir.perp().normalize_or_zero();
            let loc = pos + perp * LASER_HANDLE_DIST;
            if Circle::new(loc.x, loc.y, size).contains(&mouse) {
                return Some(HandleKind::MoveLaserBase);
            }
        }
        Object::Bird {
            rect,
            detection_range,
            ..
        } => {
            if Circle::new(
                rect.x + rect.w / 2.,
                rect.y + rect.h / 2. + detection_range,
                size,
            )
            .contains(&mouse)
            {
                return Some(HandleKind::DetectionRange);
            }
        }
        Object::Wave {
            rect,
            radius,
            angle,
            width,
        } => {
            {
                let mid = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                let offset = Vec2::from_angle(*angle) * *radius;
                let pos = mid + offset;
                if Circle::new(pos.x, pos.y, size).contains(&mouse) {
                    return Some(HandleKind::ConeAngleRadius);
                }
            }
            {
                let mid = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                let offset = Vec2::from_angle((*angle) + (*width / 2.)) * *radius;
                let pos = mid + offset;
                if Circle::new(pos.x, pos.y, size).contains(&mouse) {
                    return Some(HandleKind::ConeWidth);
                }
            }
        }
        Object::WaypointMarker { .. } => {}
        Object::Checkpoint { .. } => {}
        Object::TutorialTexture { .. } => {}
        Object::Player { .. } => {}
        Object::Heart { .. } => {}
        Object::Banana { .. } => {}
        Object::Strawberry { .. } => {}
    }
    None
}

pub fn draw_handles(obj: &Object, mouse: Vec2, size: f32, zoom: f32) {
    let weight = 1.0 / zoom;
    match obj {
        Object::Rect { rect, .. } => {
            for (_handle, rect) in get_handle_rects(*rect, size) {
                let color = if on_handle(rect, mouse) { RED } else { WHITE };
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
                draw_rectangle_lines_ex(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    weight,
                    DrawRectangleParams {
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
        }
        Object::Laser {
            rail_start,
            rail_end,
            rel,
        } => {
            let color1 = if Circle::new(rail_start.x, rail_start.y, size).contains(&mouse) {
                RED
            } else {
                WHITE
            };
            draw_circle(rail_start.x, rail_start.y, size / CIRCLE_DIV, color1);
            let color2 = if Circle::new(rail_end.x, rail_end.y, size).contains(&mouse) {
                RED
            } else {
                WHITE
            };
            draw_circle(rail_end.x, rail_end.y, size / CIRCLE_DIV, color2);
            let pos = rail_start.lerp(*rail_end, *rel);
            let dir = *rail_end - *rail_start;
            let perp = dir.perp().normalize_or_zero();
            let loc = pos + perp * LASER_HANDLE_DIST;
            let color2 = if Circle::new(loc.x, loc.y, size).contains(&mouse) {
                RED
            } else {
                WHITE
            };
            draw_circle(loc.x, loc.y, size / CIRCLE_DIV, color2);
        }
        Object::Bird {
            rect,
            detection_range,
            ..
        } => {
            let (x, y) = (rect.x + rect.w / 2., rect.y + rect.h / 2. + detection_range);
            let color = if Circle::new(x, y, size).contains(&mouse) {
                RED
            } else {
                WHITE
            };
            draw_circle(x, y, size / CIRCLE_DIV, color);
        }
        Object::Wave {
            rect,
            radius,
            angle,
            width,
        } => {
            {
                let mid = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                let offset = Vec2::from_angle(*angle) * *radius;
                let pos = mid + offset;
                let color = if Circle::new(pos.x, pos.y, size).contains(&mouse) {
                    RED
                } else {
                    WHITE
                };
                draw_circle(pos.x, pos.y, size / CIRCLE_DIV, color);
            }
            {
                let mid = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                let offset = Vec2::from_angle((*angle) + (*width / 2.)) * *radius;
                let pos = mid + offset;
                let color = if Circle::new(pos.x, pos.y, size).contains(&mouse) {
                    RED
                } else {
                    WHITE
                };
                draw_circle(pos.x, pos.y, size / CIRCLE_DIV, color);
            }
        }
        Object::WaypointMarker { .. } => {}
        Object::Checkpoint { .. } => {}
        Object::Player { .. } => {}
        Object::Heart { .. } => {}
        Object::Banana { .. } => {}
        Object::Strawberry { .. } => {}
        Object::TutorialTexture { .. } => {}
    }
}

pub fn resized(start: Rect, handle_kind: HandleKind, delta: Vec2) -> Rect {
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
        _ => unreachable!(),
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

pub fn on_handle(handle: Rect, mouse: Vec2) -> bool {
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
