pub fn path_join(segments: Vec<&str>) -> String {
    segments.join(std::path::MAIN_SEPARATOR_STR)
}

pub fn sprite(s: &str) -> String {
    return path_join(vec!["sprites", s]);
}

pub fn font(s: &str) -> String {
    return path_join(vec!["fonts", s]);
}
