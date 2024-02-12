use macroquad::{prelude::*, ui::{hash, root_ui, widgets, Skin}};

pub fn show_popup(theme: &Skin, msg: String) {
    root_ui().push_skin(&theme);
    let label_size = root_ui().calc_size(&msg);
    widgets::Window::new(
        hash!(),
        Vec2::new((screen_width() / 2.) - (label_size.x/2.+50.), screen_height() / 2. - 75.),
        Vec2::new(label_size.x+100., 150.),
    )
    .label("Connecting")
    .titlebar(false)
    .ui(&mut root_ui(), |ui| {
        ui.label(Vec2::new(50., 50.), &msg);
    });
    root_ui().pop_skin();
}