extern crate tcod;

use tcod::console::*;
use tcod::colors;

const SCREEN_HEIGHT: i32 = 80;
const SCREEN_WIDTH: i32 = 50;
const LIMIT_FPS: i32 = 20;

fn handle_keys(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
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
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,

        _ => {},
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

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    while !root.window_closed() {
        con.set_default_foreground(colors::WHITE);
        con.clear();
        con.put_char(player_x, player_y, '@', BackgroundFlag::None);
        blit(
            &mut con, 
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut root,
            (0, 0),
            1.0,
            1.0
        );
        root.flush();

        // Handle keys and exit if needed.
        let exit = handle_keys(&mut root, &mut player_x, &mut player_y);
        if exit {
            break;
        }

    }
}
