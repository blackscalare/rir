pub fn center(screen_width: i32, screen_height: i32, width: i32, size: i32) -> (i32, i32) {
    (
        (screen_width / 2) - (width + size),
        (screen_height / 2) - size,
    )
}
