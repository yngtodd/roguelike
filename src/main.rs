extern crate tcod;

use tcod::colors::{self, Color};
use tcod::console::*;

const SCREEN_HEIGHT: i32 = 80;
const SCREEN_WIDTH: i32 = 50;
const LIMIT_FPS: i32 = 20;

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

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        // Move by a given amount
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

fn handle_keys(root: &mut Root, player: &mut Object) -> bool {
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
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),

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

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
    let mut objects = [player, npc];

    while !root.window_closed() {
        con.clear();

        for object in &objects {
            object.draw(&mut con);
        }

        blit(
            &mut con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut root,
            (0, 0),
            1.0,
            1.0,
        );

        root.flush();

        // Handle keys and exit if needed.
        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player);
        if exit {
            break;
        }
    }
}
