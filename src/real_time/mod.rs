enum Update {
    Mouse(MouseMovingUpdate),
}
struct Updates {
    user: i32,
    updates: Vec<Update>,
}
struct MouseMovingUpdate {
    x: i32,
    y: i32,
}
