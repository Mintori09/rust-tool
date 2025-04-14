pub fn parse_vtt(path: &str) -> Vec<String> {
    let content = std::fs::read_to_string(path).expect("Read failed");
    content
        .split("\n\n")
        .filter(|block| block.contains("-->"))
        .map(|b| b.to_string())
        .collect()
}
pub fn clean_vtt_line(line: &str) -> String {
    // Bỏ phần timestamp
    let without_timing = line
        .lines()
        .skip(2) // bỏ 2 dòng đầu (timestamp + trống)
        .collect::<Vec<_>>()
        .join(" ");

    // Bỏ các tag kiểu <00:00:03.850> và <c> </c>
    let cleaned = regex::Regex::new(r"<[^>]+>")
        .unwrap()
        .replace_all(&without_timing, "");

    cleaned.to_string()
}
