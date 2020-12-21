mod helpers;
use helpers::get_lines;

use self::Movement::*;
use scan_fmt::scan_fmt;

#[derive(Debug)]
struct ShipState {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

#[derive(Clone, Copy, Debug)]
enum Movement {
    Forward(i64),
    MoveX(i64),
    MoveY(i64),
    Turn(i64),
}

fn parse_movement(input: &str) -> Movement {
    if let Ok((inst, value)) = scan_fmt!(input, "{[FNSEWLR]}{d}", String, i64) {
        match &inst[..] {
            "F" => Forward(value),
            "N" => MoveY(value),
            "S" => MoveY(-value),
            "E" => MoveX(value),
            "W" => MoveX(-value),
            "L" => Turn(value),
            "R" => Turn(-value),
            _ => panic!("Unknown instruction letter: {}", inst),
        }
    } else {
        panic!("Couldn't parse movement: {}", input);
    }
}

fn apply_movement(mvt: Movement, state: ShipState) -> ShipState {
    match mvt {
        Forward(dist) => ShipState {
            x: state.x + state.dx * dist,
            y: state.y + state.dy * dist,
            dx: state.dx,
            dy: state.dy,
        },
        MoveX(dist) => ShipState {
            x: state.x + dist,
            y: state.y,
            dx: state.dx,
            dy: state.dy,
        },
        MoveY(dist) => ShipState {
            x: state.x,
            y: state.y + dist,
            dx: state.dx,
            dy: state.dy,
        },
        Turn(angle) => {
            let normalized_angle = angle.rem_euclid(360);
            match normalized_angle {
                0 => state,
                90 => ShipState {
                    x: state.x,
                    y: state.y,
                    dx: -state.dy,
                    dy: state.dx,
                },
                180 => ShipState {
                    x: state.x,
                    y: state.y,
                    dx: -state.dx,
                    dy: -state.dy,
                },
                270 => ShipState {
                    x: state.x,
                    y: state.y,
                    dx: state.dy,
                    dy: -state.dx,
                },
                _ => panic!("weird angle: {}", angle),
            }
        }
    }
}

fn apply_waypoint_move(mvt: Movement, state: ShipState) -> ShipState {
    match mvt {
        Forward(dist) => ShipState {
            x: state.x + state.dx * dist,
            y: state.y + state.dy * dist,
            dx: state.dx,
            dy: state.dy,
        },
        MoveX(dist) => ShipState {
            x: state.x,
            y: state.y,
            dx: state.dx + dist,
            dy: state.dy,
        },
        MoveY(dist) => ShipState {
            x: state.x,
            y: state.y,
            dx: state.dx,
            dy: state.dy + dist,
        },
        Turn(angle) => {
            let normalized_angle = angle.rem_euclid(360);
            match normalized_angle {
                0 => state,
                90 => ShipState {
                    x: state.x,
                    y: state.y,
                    dx: -state.dy,
                    dy: state.dx,
                },
                180 => ShipState {
                    x: state.x,
                    y: state.y,
                    dx: -state.dx,
                    dy: -state.dy,
                },
                270 => ShipState {
                    x: state.x,
                    y: state.y,
                    dx: state.dy,
                    dy: -state.dx,
                },
                _ => panic!("weird angle: {}", angle),
            }
        }
    }
}


fn apply_moves_basic(moves: &[Movement]) -> ShipState {
    let mut state = ShipState {
        x: 0,
        y: 0,
        dx: 1,
        dy: 0,
    };
    for &mvt in moves {
        state = apply_movement(mvt, state);
    }
    state
}

fn apply_moves_waypoint(moves: &[Movement]) -> ShipState {
    let mut state = ShipState {
        x: 0,
        y: 0,
        dx: 10,
        dy: 1,
    };
    for &mvt in moves {
        state = apply_waypoint_move(mvt, state);
        println!("{:?}", state);
    }
    state
}

fn main() {
    let lines = get_lines("input.txt");
    let moves: Vec<Movement> = lines.iter().map(|line| parse_movement(&line)).collect();
    let newstate = apply_moves_basic(&moves);
    let dist = newstate.x.abs() + newstate.y.abs();
    println!("Distance moved with basic instructions: {}", dist);

    let newstate = apply_moves_waypoint(&moves);
    let dist = newstate.x.abs() + newstate.y.abs();
    println!("Distance moved with waypoint instructions: {}", dist);
}

#[test]
fn test_example() {
    let moves = vec![Forward(10), MoveY(3), Forward(7), Turn(-90), Forward(11)];
    let state = apply_moves_basic(&moves);
    assert_eq!(state.x, 17);
    assert_eq!(state.y, -8);

    let state = apply_moves_waypoint(&moves);
    assert_eq!(state.x, 214);
    assert_eq!(state.y, -72);
}
