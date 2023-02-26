use x11rb::{
    connection::Connection,
    protocol::xproto::{Atom, AtomEnum, ConnectionExt, Screen, Window},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let (conn, screen) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen];

    let focused_window = find_active_window(&conn, screen).unwrap();
    let focused_geom = conn.get_geometry(focused_window)?.reply()?;

    create_window(
        &conn,
        screen,
        focused_geom.width,
        focused_geom.height,
        focused_geom.x,
        focused_geom.y,
    )?;

    Ok(())
}

/// A magical function that magically gets the active window using magic
/// (no idea how this works, I copied it from reddit)
fn find_active_window(conn: &impl Connection, screen: &Screen) -> Option<Window> {
    let root = screen.root;
    let net_active_window = conn
        .intern_atom(true, b"_NET_ACTIVE_WINDOW")
        .unwrap()
        .reply()
        .unwrap()
        .atom;
    let window: Atom = AtomEnum::WINDOW.into();
    let active_window = conn
        .get_property(false, root, net_active_window, window, 0, 1)
        .expect("Failed to get X11 property")
        .reply()
        .expect("Failed to receive X11 property reply");

    if active_window.format == 32 && active_window.length == 1 {
        active_window
            .value32()
            .expect("Invalid message. Expected value with format = 32")
            .next()
    } else {
        Some(
            conn.get_input_focus()
                .expect("Failed to get input focus")
                .reply()
                .expect("Failed to receive X11 input focus")
                .focus,
        )
    }
}

/// A magical function that magically creates a window using magic
/// (no idea how this works, I copied this from github)
fn create_window(
    conn: &impl Connection,
    screen: &Screen,
    width: u16,
    height: u16,
    x: i16,
    y: i16,
) -> Result<()> {
    let root = screen.root;
    let win_id = conn.generate_id()?;

    conn.create_window(
        x11rb::COPY_DEPTH_FROM_PARENT,
        win_id,
        root,
        x,
        y,
        width,
        height,
        0,
        x11rb::protocol::xproto::WindowClass::INPUT_OUTPUT,
        0,
        &x11rb::protocol::xproto::CreateWindowAux::new()
            .background_pixel(screen.white_pixel)
            .override_redirect(Some(1)),
    )?;

    conn.map_window(win_id)?;
    conn.flush()?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok(())
}
