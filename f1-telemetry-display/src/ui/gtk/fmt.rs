pub fn format_gear(gear: i8) -> &str {
    &format!("{} ", match gear {
        -1 => "R",
        0 => "N",
        _ => gear
    })
}