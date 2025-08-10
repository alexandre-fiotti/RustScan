#[derive(Debug, Clone)]
pub enum ResultsMsg {
    AppendLine(String),
    AppendLines(Vec<String>),
    Clear,
    ScrollUp(usize),
    ScrollDown(usize),
    ScrollToTop,
    ScrollToBottom,
}
