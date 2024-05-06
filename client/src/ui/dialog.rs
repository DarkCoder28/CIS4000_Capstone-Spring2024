use common::ClientState;
use macroquad::{
    input::{is_key_pressed, KeyCode},
    math::vec2,
    time::get_time,
    ui::{root_ui, widgets},
    window::{screen_height, screen_width},
};
use tracing::{error, info};

use crate::quest_data::{get_next_questline_id, Questline};

// #[instrument(name = "render_dialog")]
pub fn render_dialog(
    questlines: &Vec<Questline>,
    open_time: f64,
    state: &mut ClientState,
) -> (bool, f64) {
    let questline = questlines
        .iter()
        .find(|ql| ql.id == state.current_questline_id);
    if questline.is_some() {
        let questline = questline.unwrap();
        let quest_index = questline
            .quests
            .iter()
            .position(|q| q.quest_id.is_some() && q.quest_id.unwrap() == state.current_quest_id);
        if quest_index.is_some() {
            let quest_index = quest_index.unwrap();
            let dialog_data = questline.quests.get(quest_index..);
            if dialog_data.is_some() {
                let dialog_data = dialog_data.unwrap();
                // info!("Questline: {}\t\tOffset: {}", &state.current_questline_id, &state.dialog_offset);
                if dialog_data.len() < (state.dialog_offset + 1) as usize {
                    // Mark current quest complete
                    if !state.complete_quest_ids.contains(&(state.current_questline_id+state.current_quest_id)) {
                        state.complete_quest_ids.push(state.current_questline_id+state.current_quest_id);
                    }
                    // Get new questline id
                    state.current_questline_id = get_next_questline_id(questlines, state.current_questline_id);
                    let new_questline = questlines.iter().find(|ql|ql.id == state.current_questline_id);
                    if new_questline.is_some() {
                        // Set quest to 0
                        state.current_quest_id = 0;
                        state.dialog_offset = 0;
                        return (false, get_time());
                    }
                    // increment_quest(questlines, state);
                    // state.current_questline_id =
                    //     get_next_questline_id(questlines, state.current_questline_id);
                    // let new_questline = questlines
                    //     .iter()
                    //     .find(|ql| ql.id == state.current_questline_id)
                    //     .unwrap();
                    // let next_quest = new_questline.quests.first().unwrap();
                    // state.current_quest_id = next_quest
                    //     .quest_id
                    //     .unwrap_or(state.current_questline_id * 10);
                    return (false, get_time());
                }
                let current = &dialog_data[state.dialog_offset as usize];
                // Render Dialog
                widgets::Window::new(
                    0b011001000110100101100001011011000110111101100111,
                    vec2(screen_width() * 0.05, screen_height() * 0.7),
                    vec2(screen_width() * 0.9, screen_height() * 0.25),
                )
                .label(&current.speaker)
                .titlebar(false)
                .close_button(false)
                .movable(false)
                .ui(&mut root_ui(), |ui| {
                    ui.label(None, &current.speaker);
                    ui.label(None, "");
                    // ui.label(vec2(50., 50.), &current.dialog);
                    // let label_size = ui.calc_size(&current.dialog);
                    let label_text = &current.dialog;
                    let mut current_width = 0.;
                    let sliced_text = label_text.split_whitespace();
                    let mut wrapped_text = String::new();
                    for word in sliced_text {
                        let mut word = String::from(word);
                        word.push(' ');
                        let text_size = ui.calc_size(&word);
                        current_width += text_size.x;
                        if current_width > screen_width() * 0.8 {
                            current_width = text_size.x;
                            wrapped_text.push('\n');
                        }
                        wrapped_text.push_str(&word);
                    }
                    for line in wrapped_text.split('\n') {
                        ui.label(None, line);
                    }
                    // ui.label(None, &wrapped_text);
                    // Label::new(&wrapped_text)
                    //     // .position(vec2(screen_width() * 0.01, screen_height() * 0.01))
                    //     .multiline(label_size.y)
                    //     .size(vec2(screen_width() * 0.88, screen_height() * 0.23))
                    //     .ui(ui);
                });
                root_ui().move_window(
                    0b011001000110100101100001011011000110111101100111,
                    vec2(screen_width() * 0.9, screen_height() * 0.25),
                );
                //
                if is_key_pressed(KeyCode::Space) && get_time() - open_time > 1f64 {
                    if current.quest_id.is_some() {
                        state.dialog_offset = 0;
                        state.current_quest_id = current.quest_id.unwrap();
                        info!("Quest updated: {}", state.current_quest_id);
                        return (true, get_time());
                    }
                    return (false, get_time());
                } else {
                    return (false, open_time);
                }
            }
        }
    } else {
        error!("Cant find quest with id '{}'.", &state.current_quest_id);
    }
    (false, get_time())
}
