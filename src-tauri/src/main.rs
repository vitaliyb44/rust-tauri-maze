#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{thread, time::Duration};

use rand::Rng;
use serde::Serialize;
use tauri::{async_runtime::spawn, AppHandle, Window};

static mut STOP_THREAD: bool = false;

#[derive(Clone, Copy, Serialize)]
struct MazeCell {
    wall_n: bool,
    wall_w: bool,
    wall_e: bool,
    wall_s: bool,
    visited: bool,
}

#[derive(Serialize, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn generate_new_maze(
    _app: AppHandle,
    window: Window,
    rows: usize,
    cols: usize,
    _enter_x: Option<usize>,
    _enter_y: Option<usize>,
    step_flag: bool,
    sleep_duration: u64,
) {
    let maze_cell = MazeCell {
        wall_n: true,
        wall_e: true,
        wall_w: true,
        wall_s: true,
        visited: false,
    };
    let mut maze: Vec<Vec<MazeCell>> = vec![vec![maze_cell; cols]; rows];

    spawn(async move {
        let mut rng = rand::thread_rng();

        let maze_start_position = Point {
            x: rng.gen_range(0..(cols as i32) - 1),
            y: rng.gen_range(0..(rows as i32) - 1),
        };
        let mut maze_path: Vec<Point> = vec![maze_start_position];

        'maze: while !maze_path.is_empty() {
            unsafe {
                if STOP_THREAD {
                    break 'maze;
                }
            }
            let current_point = maze_path.last().unwrap();
            maze[current_point.y as usize][current_point.x as usize].visited = true;
            let valid_neighbors =
                get_valid_neighbors(current_point.x, current_point.y, rows, cols, &maze);
            let has_no_neighbor = valid_neighbors.is_empty();

            if has_no_neighbor {
                maze_path.pop();
                continue;
            }

            let new_point = &valid_neighbors[rng.gen_range(0..valid_neighbors.len())];

            carve_maze_path(current_point, new_point, &mut maze);

            if step_flag {
                window
                    .emit(
                        "add_maze_point",
                        (
                            (maze[new_point.y as usize][new_point.x as usize], *new_point),
                            (
                                maze[current_point.y as usize][current_point.x as usize],
                                *current_point,
                            ),
                        ),
                    )
                    .unwrap();
                thread::sleep(Duration::from_millis(sleep_duration));
            }
            maze_path.push(*new_point);
        }
        window.emit("maze_finished", maze).unwrap();
    });
}

fn carve_maze_path(current_point: &Point, new_point: &Point, maze: &mut Vec<Vec<MazeCell>>) {
    // difference
    //    N-1
    // W-1  E+1
    //    S+1
    match new_point.x - current_point.x {
        1 => {
            maze[current_point.y as usize][current_point.x as usize].wall_e = false;
            maze[new_point.y as usize][new_point.x as usize].wall_w = false;
        }
        -1 => {
            maze[current_point.y as usize][current_point.x as usize].wall_w = false;
            maze[new_point.y as usize][new_point.x as usize].wall_e = false;
        }
        _ => (),
    }
    match new_point.y - current_point.y {
        1 => {
            maze[current_point.y as usize][current_point.x as usize].wall_s = false;
            maze[new_point.y as usize][new_point.x as usize].wall_n = false;
        }
        -1 => {
            maze[current_point.y as usize][current_point.x as usize].wall_n = false;
            maze[new_point.y as usize][new_point.x as usize].wall_s = false;
        }
        _ => (),
    }
}

fn get_valid_neighbors(
    x: i32,
    y: i32,
    rows: usize,
    cols: usize,
    maze: &Vec<Vec<MazeCell>>,
) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::with_capacity(4);
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (d_x, d_y) in directions {
        if !in_bounds(x + d_x, y + d_y, rows as i32, cols as i32) {
            continue;
        }
        if cell_visited((x + d_x) as usize, (y + d_y) as usize, &maze) {
            continue;
        }
        neighbors.push(Point {
            x: x + d_x,
            y: y + d_y,
        });
    }
    neighbors
}
fn in_bounds(x: i32, y: i32, rows: i32, cols: i32) -> bool {
    if (x <= cols - 1 && x >= 0) && (y <= rows - 1 && y >= 0) {
        true
    } else {
        false
    }
}
fn cell_visited(x: usize, y: usize, maze: &Vec<Vec<MazeCell>>) -> bool {
    if maze[y][x].visited {
        true
    } else {
        false
    }
}

#[tauri::command]
async fn create_maze(
    app: AppHandle,
    window: Window,
    rows: usize,
    cols: usize,
    maze_enter_x: Option<usize>,
    maze_enter_y: Option<usize>,
    step_flag: bool,
    sleep_duration: u64,
) -> Result<(), ()> {
    unsafe {
        STOP_THREAD = false;
    }
    window.emit("maze_generate_start", {}).unwrap();
    generate_new_maze(
        app,
        window,
        rows,
        cols,
        maze_enter_x,
        maze_enter_y,
        step_flag,
        sleep_duration,
    );

    Ok(())
}
#[tauri::command]
fn pause_maze_generation() {
    unsafe {
        STOP_THREAD = true;
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_maze, pause_maze_generation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
