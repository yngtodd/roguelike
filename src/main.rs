extern crate tcod;

use tcod::colors::{self, Color};
use tcod::console::*;

// Size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// Size of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

// Maximum number of frames per second
const LIMIT_FPS: i32 = 20;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    blocked_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { blocked: false, blocked_sight: false }
    }

    pub fn wall() -> Self {
        Tile { blocked: true, blocked_sight: true }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }
}

fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y1 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        if !map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            // Move by a given amount
            self.x += dx;
            self.y += dy;
        } else {
            println!("Movement is blocked!")
        }
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

fn make_map() -> Map {
    // Fill with "blocked" tiles
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(50, 15, 10, 15);

    create_room(room1, &mut map);
    create_room(room2, &mut map);

    map
}

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map) {
    // go through all tiles, and set their background color
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = map[x as usize][y as usize].blocked_sight;
            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    // draw all objects in the list
    for object in objects {
        object.draw(con);
    }

    // blit the contents of "con" to the root console
    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
}

fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt + Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }

        // Exit the game
        Key { code: Escape, .. } => return true,

        // Movement keys
        Key { code: Up, .. } => player.move_by(0, -1, map),
        Key { code: Down, .. } => player.move_by(0, 1, map),
        Key { code: Left, .. } => player.move_by(-1, 0, map),
        Key { code: Right, .. } => player.move_by(1, 0, map),

        _ => {}
    }

    false
}

fn main() {
    let mut root = Root::initializer()
        .font("assets/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_HEIGHT, SCREEN_WIDTH)
        .title("Rust/libtocod tutorial")
        .init();

    tcod::system::set_fps(LIMIT_FPS);
    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let player = Object::new(25, 23, '@', colors::WHITE);
   
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);

    let mut objects = [player, npc];

    let map = make_map();  

    while !root.window_closed() {
        // Clear the screen of the previous frame
        con.clear();

        // Render the screen
        render_all(&mut root, &mut con, &objects, &map);

        root.flush();

        // Handle keys and exit if needed.
        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break;
        }
    }
}
