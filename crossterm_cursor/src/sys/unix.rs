/// Get the cursor position based on the current platform.
#[cfg(unix)]
pub fn get_cursor_position() -> (u16, u16) {
    if let Ok(pos) = pos() {
        pos
    } else {
        (0, 0)
    }
}
