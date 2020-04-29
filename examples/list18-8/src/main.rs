fn main() {
    let some_option_value: Option<i32> = None;
    // pattern `None` not covered
    // `let` bindings require an "irrefutable pattern"
    // let Some(x) = some_option_value;
}
