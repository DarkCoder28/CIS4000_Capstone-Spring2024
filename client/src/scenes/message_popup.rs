use macroquad::{prelude::*, ui::{root_ui, widgets, Skin}};

pub fn show_popup(theme: &Skin, msg: String) {
    root_ui().push_skin(&theme);
    let label_size = root_ui().calc_size(&msg);
    widgets::Window::new(
        0b0100001101101111011011100110111001100101011000110111010001101001,
        Vec2::new((screen_width() / 2.) - (label_size.x/2.+50.), screen_height() / 2. - 75.),
        Vec2::new(label_size.x+100., 150.),
    )
    .label("Connecting")
    .titlebar(false)
    .ui(&mut root_ui(), |ui| {
        ui.label(Vec2::new(50., 50.), &msg);
    });
    root_ui().move_window(0b0100001101101111011011100110111001100101011000110111010001101001, Vec2::new((screen_width() / 2.) - (label_size.x/2.+50.), screen_height() / 2. - 75.));
    root_ui().pop_skin();
}