use super::*;
pub fn parse(s: &str, app: &mut App) {
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
        fn parse_int(s: &str) -> i32 {
            s.parse().expect("Expected int")
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
            "PF_BOUND" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                let w = parse_float(split[3]);
                let h = parse_float(split[4]);
                app.push_object(Object::Rect {
                    rect: Rect::new(x - w / 2., y - h / 2., w, h),
                    kind: RectKind::PathfindingBound,
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
                let canleave = parse_int(split[4]);
                app.push_object(Object::Bird {
                    rect: Rect::new(x - 50. / 2., y - 50. / 2., 50., 50.),
                    detection_range: detrange,
                    kind: BirdKind::Regular,
                    can_leave_spawn: canleave != 0,
                });
            }
            "PATHFINDING_BIRD" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                let detrange = parse_float(split[3]);
                let canleave = parse_int(split[4]);
                app.push_object(Object::Bird {
                    rect: Rect::new(x - 50. / 2., y - 50. / 2., 50., 50.),
                    detection_range: detrange,
                    kind: BirdKind::Pathfinding,
                    can_leave_spawn: canleave != 0,
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
                let radius = parse_float(split[3]);
                let angle = parse_float(split[4]);
                let width = parse_float(split[5]);
                app.push_object(Object::Wave {
                    rect: Rect::new(x - 40. / 2., y - 40. / 2., 40., 40.),
                    radius,
                    angle,
                    width,
                });
            }
            "WAYPOINT" => {
                let x = parse_float(split[1]);
                let y = parse_float(split[2]);
                let waypoint_id = parse_int(split[3]);
                app.push_object(Object::WaypointMarker {
                    rect: Rect::new(x - 40. / 2., y - 40. / 2., WAYPOINT_SIZE, WAYPOINT_SIZE),
                    waypoint_id,
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

pub fn serialise(app: &App) -> Result<Option<String>, std::fmt::Error> {
    let mut s = String::new();
    let mut player = 0;
    let mut tree = BTreeMap::new();
    tree.extend(&app.objects);
    let mut set = HashSet::new();
    let mut set_err = false;
    for object in tree.values() {
        match object {
            Object::Rect { rect, kind } => {
                let typ = match kind {
                    RectKind::Regular => "REGULAR",
                    RectKind::AntiTp => "ANTI_TP",
                    RectKind::Tar => "TAR",
                    RectKind::Hot => "HOT",
                    RectKind::PathfindingBound => "PF_BOUND",
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
                kind,
                can_leave_spawn,
            } => match kind {
                BirdKind::Regular => {
                    writeln!(
                        s,
                        "BIRD {} {} {} {}",
                        rect.x + rect.w / 2.,
                        rect.y + rect.h / 2.,
                        detection_range,
                        (*can_leave_spawn) as i32,
                    )?;
                }
                BirdKind::Pathfinding => {
                    writeln!(
                        s,
                        "PATHFINDING_BIRD {} {} {} {}",
                        rect.x + rect.w / 2.,
                        rect.y + rect.h / 2.,
                        detection_range,
                        (*can_leave_spawn) as i32,
                    )?;
                }
            },

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
                radius,
                angle,
                width,
            } => {
                writeln!(
                    s,
                    "WAVE {} {} {} {} {}",
                    rect.x + rect.w / 2.,
                    rect.y + rect.h / 2.,
                    radius,
                    angle,
                    width,
                )?;
            }
            Object::WaypointMarker { rect, waypoint_id } => {
                if !set.insert(waypoint_id) {
                    eprintln!(
                        "\x1b[31mError\x1b[0m: Found 2+ waypoint markers with same ID: {}",
                        waypoint_id
                    );
                    set_err = true;
                }
                writeln!(
                    s,
                    "WAYPOINT {} {} {}",
                    rect.x + rect.w / 2.,
                    rect.y + rect.h / 2.,
                    waypoint_id,
                )?;
            }
        }
    }
    if player != 1 {
        eprintln!("\x1b[31mError\x1b[0m: {} player objects found", player);
        return Ok(None);
    }
    if set_err {
        return Ok(None);
    }
    Ok(Some(s))
}
