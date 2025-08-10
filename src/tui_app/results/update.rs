use super::message::ResultsMsg;
use super::model::ResultsModel;

pub fn update_results(model: &mut ResultsModel, msg: ResultsMsg) {
    match msg {
        ResultsMsg::AppendLine(line) => model.push_line(line),
        ResultsMsg::AppendLines(lines) => model.push_lines(lines),
        ResultsMsg::Clear => model.clear(),
        ResultsMsg::ScrollUp(n) => model.scroll_up(n),
        ResultsMsg::ScrollDown(n) => model.scroll_down(n),
        ResultsMsg::ScrollToTop => model.scroll_to_top(),
        ResultsMsg::ScrollToBottom => model.scroll_to_bottom(),
    }
}
