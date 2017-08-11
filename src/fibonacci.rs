#[no_mangle]
pub extern fn calc(x: i32) -> i32 {
    if x <= 2 {
        return 1;
    }
    return calc(x - 1) + calc(x - 2);
}