use macroquad::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::sync::atomic::{AtomicU32, Ordering};

const LASER_WIDTH: f32 = 40.;
const LASER_HEIGHT: f32 = 20.;
const PLAYER_WIDTH: f32 = 48.;
const PLAYER_HEIGHT: f32 = 96.;
const HANDLE_SIZE: f32 = 10.0;
const SIDEBAR_SIZE: f32 = 40.;
const PATCH_SIZE: f32 = 30.;
const ITEM_HEIGHT: f32 = 50.;
const CONTEXT_MENU_HEIGHT: f32 = 140.;
const LASER_HANDLE_DIST: f32 = 20.;
const CIRCLE_DIV: f32 = 2.0;

fn parse(s: &str, app: &mut App) {
    let file = match std::fs::read_to_string(s) {
        Ok(f) => f,
        Err(_e) => {
            eprintln!("Failed to read file, falling back to empty file");
            return;
        }
    };
    for line in file.lines() {
        if line.is_empty() {
            continue;
        }
        let split = line.split(" ").collect::<Vec<_>>();
        fn parse_float(s: &str) -> f32 {
            s.parse().expect("Expected float")
        }
        match split[0] {
            "REGULAR" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                let w = parse_float(split[3]);
                let h = parse_float(split[4]);
                app.push_object(Object::Rect {
                    rect: Rect::new(x - w / 2., y - h / 2., w, h),
                    // rect: Rect::new(x, y, w, h),
                    kind: RectKind::Regular,
                });
            }
            "ANTI_TP" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                let w = parse_float(split[3]);
                let h = parse_float(split[4]);
                app.push_object(Object::Rect {
                    rect: Rect::new(x - w / 2., y - h / 2., w, h),
                    // rect: Rect::new(x, y, w, h),
                    kind: RectKind::AntiTp,
                });
            }
            "TAR" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                let w = parse_float(split[3]);
                let h = parse_float(split[4]);
                app.push_object(Object::Rect {
                    rect: Rect::new(x - w / 2., y - h / 2., w, h),
                    // rect: Rect::new(x, y, w, h),
                    kind: RectKind::Tar,
                });
            }
            "HOT" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                let w = parse_float(split[3]);
                let h = parse_float(split[4]);
                app.push_object(Object::Rect {
                    rect: Rect::new(x - w / 2., y - h / 2., w, h),
                    // rect: Rect::new(x, y, w, h),
                    kind: RectKind::Hot,
                });
            }
            "LASER" => {
                let x1 = parse_float(split[1]);
                let y1 = parse_float(split[2]);
                let x2 = parse_float(split[3]);
                let y2 = parse_float(split[4]);
                let rel = parse_float(split[5]);
                app.push_object(Object::Laser {
                    rail_start: Vec2::new(x1, y1),
                    rail_end: Vec2::new(x2, y2),
                    rel,
                });
            }
            "PLAYER" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                app.push_object(Object::Player {
                    pos: Vec2::new(x - PLAYER_WIDTH / 2., y - PLAYER_HEIGHT / 2.),
                })
            }
            "BIRD" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                let detrange = parse_float(split[3]);
                app.push_object(Object::Bird {
                    rect: Rect::new(x - 50. / 2., y - 50. / 2., 50., 50.),
                    detection_range: detrange,
                });
            }
            "HEART" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                app.push_object(Object::Heart {
                    rect: Rect::new(x - 48. / 2., y - 48. / 2., 48., 48.),
                });
            }
            "BANANA" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                app.push_object(Object::Banana {
                    rect: Rect::new(x - 48. / 2., y - 48. / 2., 48., 48.),
                });
            }
            "STRAWBERRY" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                app.push_object(Object::Strawberry {
                    rect: Rect::new(x - 48. / 2., y - 48. / 2., 48., 48.),
                });
            }
            "CHECKPOINT" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                app.push_object(Object::Checkpoint {
                    rect: Rect::new(x - 60. / 2., y, 60., 60.),
                });
            }
            "WAVE" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                let detrange = parse_float(split[3]);
                app.push_object(Object::Wave {
                    rect: Rect::new(x - 40. / 2., y - 40. / 2., 40., 40.),
                    detection_range: detrange,
                });
            }
            "TUTORIAL_OBJ" => {
                let id = match split[1] {
                    "checkpoint" => "checkpoint",
                    "close_window" => "close_window",
                    "bird" => "bird",
                    "laser" => "laser",
                    "wave" => "wave",
                    "framerate" => "framerate",
                    "fruits" => "fruits",
                    "heart" => "heart",
                    "jump" => "jump",
                    "move" => "move",
                    "reset" => "reset",
                    "hot" => "hot",
                    "pink" => "pink",
                    "tar1" => "tar1",
                    "tar2" => "tar2",
                    "teleport" => "teleport",
                    "walljump" => "walljump",
                    "congrats" => "congrats",
                    "portal" => "portal",
                    _ => panic!("Invalid tutorial object found: {}", split[1]),
                };
                let x = parse_float(split[2]);
                let y = parse_float(split[3]);
                let w = 116.0;
                let h = 88.0;
                app.push_object(Object::TutorialTexture {
                    id,
                    rect: Rect::new(x - w / 2., y - h / 2., w, h),
                });
            }
            _ => {
                eprintln!("Invalid object found in level file: {}", split[0]);
                std::process::exit(1);
            }
        }
    }
}

fn serialise(app: &App) -> Result<String, std::fmt::Error> {
    let mut s = String::new();
    let mut player = 0;
    for object in app.objects.values() {
        match object {
            Object::Rect { rect, kind } => {
                let typ = match kind {
                    RectKind::Regular => "REGULAR",
                    RectKind::AntiTp => "ANTI_TP",
                    RectKind::Tar => "TAR",
                    RectKind::Hot => "HOT",
                };
                writeln!(
                    s,
                    "{} {} {} {} {}",
                    typ,
                    rect.x + rect.w / 2.,
                    rect.y + rect.h / 2.,
                    rect.w,
                    rect.h
                )?;
            }
            Object::Laser {
                rail_start,
                rail_end,
                rel,
            } => {
                writeln!(
                    s,
                    "LASER {} {} {} {} {}",
                    rail_start.x, rail_start.y, rail_end.x, rail_end.y, rel
                )?;
            }
            Object::Player { pos } => {
                writeln!(
                    s,
                    "PLAYER {} {}",
                    pos.x + PLAYER_WIDTH / 2.,
                    pos.y + PLAYER_HEIGHT / 2.
                )?;
                player += 1;
            }
            Object::Bird {
                rect,
                detection_range,
            } => {
                writeln!(
                    s,
                    "BIRD {} {} {}",
                    rect.x + rect.w / 2.,
                    rect.y + rect.h / 2.,
                    detection_range
                )?;
            }
            Object::Heart { rect } => {
                writeln!(s, "HEART {} {}", rect.x + rect.w / 2., rect.y + rect.h / 2.)?;
            }
            Object::Banana { rect } => {
                writeln!(
                    s,
                    "BANANA {} {}",
                    rect.x + rect.w / 2.,
                    rect.y + rect.h / 2.
                )?;
            }
            Object::Strawberry { rect } => {
                writeln!(
                    s,
                    "STRAWBERRY {} {}",
                    rect.x + rect.w / 2.,
                    rect.y + rect.h / 2.
                )?;
            }
            Object::TutorialTexture { rect, id } => {
                writeln!(
                    s,
                    "TUTORIAL_OBJ {id} {} {}",
                    rect.x + rect.w / 2.,
                    rect.y + rect.h / 2.
                )?;
            }
            Object::Checkpoint { rect } => {
                writeln!(s, "CHECKPOINT {} {}", rect.x + rect.w / 2., rect.y)?;
            }
            Object::Wave {
                rect,
                detection_range,
            } => {
                writeln!(
                    s,
                    "WAVE {} {} {}",
                    rect.x + rect.w / 2.,
                    rect.y + rect.h / 2.,
                    detection_range
                )?;
            }
        }
    }
    if player != 1 {
        eprintln!("Error: {} player objects found", player);
        return Err(std::fmt::Error);
    }
    Ok(s)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RectKind {
    Regular,
    AntiTp,
    Tar,
    Hot,
}

#[derive(Clone, Copy, Debug)]
enum Object {
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
    Strawberry {
        rect: Rect,
    },
    Checkpoint {
        rect: Rect,
    },
    Wave {
        rect: Rect,
        detection_range: f32,
    },
    TutorialTexture {
        id: &'static str,
        rect: Rect,
    },
}

impl Object {
    fn same_type(&self, other: &Self) -> bool {
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
        )
    }
    fn aabb(&self) -> Rect {
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
            Object::Checkpoint { rect }
            | Object::Wave { rect, .. }
            | Object::TutorialTexture { rect, .. }
            | Object::Bird { rect, .. }
            | Object::Heart { rect }
            | Object::Banana { rect }
            | Object::Strawberry { rect } => *rect,
        }
    }
    fn grect_aabb(&self) -> Rect {
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
    fn contains(&self, pos: Vec2) -> bool {
        self.aabb().contains(pos)
    }
    fn pos(&self) -> Vec2 {
        self.aabb().point()
    }
    fn set_pos(&mut self, pos: Vec2) {
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
            Object::Checkpoint { rect }
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
    fn set_pos_centered(&mut self, pos: Vec2) {
        match self {
            Object::Rect { rect, .. } => {
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
    fn set_size(&mut self, size: Vec2) {
        match self {
            Object::Rect { rect, .. } => {
                rect.w = size.x;
                rect.h = size.y;
            }
            Object::Checkpoint { .. }
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
    fn overlaps_rect(&self, rect: Rect) -> bool {
        self.aabb().overlaps(&rect)
    }
    fn draw(&self, textures: &Textures) {
        self.draw_with_alpha(textures, 1.0);
    }
    fn draw_with_alpha(&self, textures: &Textures, alpha: f32) {
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
                detection_range,
            } => {
                let pos = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                draw_circle(
                    pos.x,
                    pos.y,
                    *detection_range,
                    Color::from_rgba(50, 168, 153, (50_f32 * alpha) as u8),
                );
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, BLACK);
            }
        }
    }

    fn context_menu_target(&self, id: u32) -> ContextMenuTarget {
        ContextMenuTarget::SingleObject { id }
    }
}

#[derive(Debug, Clone)]
struct Textures {
    bird: Texture2D,
    laser: Texture2D,
    banana: Texture2D,
    strawberry: Texture2D,
    heart: Texture2D,
    player: Texture2D,
    checkpoint: Texture2D,
    wave: Texture2D,
    tutorial_textures: HashMap<&'static str, Texture2D>,
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
    RailStart,
    RailEnd,
    MoveLaserBase,
    DetectionRange,
}

#[derive(Debug, Clone)]
enum InteractionState {
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
    textures: Textures,
}

impl App {
    async fn new() -> Self {
        let bird = load_texture("textures/bird.png").await.unwrap();
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
        }
    }

    fn push_object(&mut self, object: Object) {
        let id = new_id();
        self.objects.insert(id, object);
        self.ordering.push(id);
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
        let escape = is_key_pressed(KeyCode::Escape);
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
                        Object::Wave {
                            rect,
                            detection_range,
                        } => {
                            let pos = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                            let diff = (pos - mouse_pos_world).length();
                            *detection_range = diff;
                        }
                        Object::Bird {
                            rect,
                            detection_range,
                        } => {
                            let pos = Vec2::new(rect.x + rect.w / 2., rect.y + rect.h / 2.);
                            let diff = (pos - mouse_pos_world).length();
                            *detection_range = diff;
                        }
                        Object::Checkpoint { .. }
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
                draw_handles(object, mouse_pos_world, HANDLE_SIZE, self.camera_zoom);
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

    fn side_panel(&mut self) {
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
        /*
            Object::Rect { .. } => 0,
            Object::Laser { .. } => 1,
            Object::Player { ..} => 2,
            Object::Bird { ..} => 3,
            Object::Heart { ..} => 4,
            Object::Banana { .. } => 5,
            Object::Strawberry { ..} => 6,
        */
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
                    detection_range: 200.,
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

    fn context_menu(&mut self) {
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
    let mut args = std::env::args();
    let _name = args.next().unwrap();
    let Some(file_name) = args.next() else {
        eprintln!("Error: must give filename to open!");
        std::process::exit(1);
    };
    match std::fs::copy(&file_name, format!("{file_name}.backup")) {
        Ok(_) => {
            println!("Succesfully backed up file");
        }
        Err(e) => {
            eprintln!("Warning: Failed to back up file: {e}")
        }
    }
    if args.next().is_some() {
        eprintln!("Error: Expected only 1 arg!");
        std::process::exit(1);
    }
    let mut app = App::new().await;
    parse(&file_name, &mut app);
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
            let serialized = serialise(&app).expect("String formatting failed??");
            if let Err(e) = std::fs::write(&file_name, serialized) {
                eprintln!("Failed to save: {e}");
            } else {
                println!("Successfully saved to {file_name}");
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

fn get_active_handle_group(r: Rect, mouse: Vec2, size: f32) -> Option<HandleKind> {
    for (handle, rect) in get_handle_rects(r, size) {
        if on_handle(rect, mouse) {
            return Some(handle);
        }
    }
    None
}

fn get_active_handle(obj: &Object, mouse: Vec2, size: f32) -> Option<HandleKind> {
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
            detection_range,
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
        Object::Checkpoint { .. } => {}
        Object::TutorialTexture { .. } => {}
        Object::Player { .. } => {}
        Object::Heart { .. } => {}
        Object::Banana { .. } => {}
        Object::Strawberry { .. } => {}
    }
    None
}

fn draw_handles(obj: &Object, mouse: Vec2, size: f32, zoom: f32) {
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
            detection_range,
        } => {
            let (x, y) = (rect.x + rect.w / 2., rect.y + rect.h / 2. + detection_range);
            let color = if Circle::new(x, y, size).contains(&mouse) {
                RED
            } else {
                WHITE
            };
            draw_circle(x, y, size / CIRCLE_DIV, color);
        }
        Object::Checkpoint { .. } => {}
        Object::Player { .. } => {}
        Object::Heart { .. } => {}
        Object::Banana { .. } => {}
        Object::Strawberry { .. } => {}
        Object::TutorialTexture { .. } => {}
    }
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
