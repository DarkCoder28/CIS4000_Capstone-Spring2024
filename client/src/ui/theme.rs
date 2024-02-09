use include_dir::{include_dir, Dir};
use macroquad::{
    prelude::*,
    ui::{root_ui, Skin},
};

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets/");

pub fn generate_theme() -> Skin {
    let font_bytes = ASSETS
        .get_file("ui/MinimalPixel v2.ttf")
        .unwrap()
        .contents();
    let window_background_bytes = ASSETS
        .get_file("ui/window_background_2.png")
        .unwrap()
        .contents();
    let button_background_bytes = ASSETS
        .get_file("ui/button_background_2.png")
        .unwrap()
        .contents();
    let button_background_hovered_bytes = ASSETS
        .get_file("ui/button_hovered_background_2.png")
        .unwrap()
        .contents();
    let button_background_clicked_bytes = ASSETS
        .get_file("ui/button_clicked_background_2.png")
        .unwrap()
        .contents();
    let checkbox_background_bytes = ASSETS
        .get_file("ui/checkbox_background.png")
        .unwrap()
        .contents();
    let checkbox_hovered_background_bytes = ASSETS
        .get_file("ui/checkbox_hovered_background.png")
        .unwrap()
        .contents();
    let checkbox_clicked_background_bytes = ASSETS
        .get_file("ui/checkbox_clicked_background.png")
        .unwrap()
        .contents();
    let editbox_background_bytes = ASSETS
        .get_file("ui/editbox_background.png")
        .unwrap()
        .contents();
    let combobox_background_bytes = ASSETS
        .get_file("ui/combobox_background.png")
        .unwrap()
        .contents();



    let label_style = root_ui()
        .style_builder()
        .font(font_bytes)
        .unwrap()
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .font_size(25)
        .build();

    let window_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(window_background_bytes, Some(ImageFormat::Png)).unwrap(),
        )
        .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
        .margin(RectOffset::new(-30.0, 0.0, -30.0, 0.0))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(button_background_bytes, Some(ImageFormat::Png)).unwrap(),
        )
        .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
        .background_hovered(
            Image::from_file_with_format(button_background_hovered_bytes, Some(ImageFormat::Png))
                .unwrap(),
        )
        .background_clicked(
            Image::from_file_with_format(button_background_clicked_bytes, Some(ImageFormat::Png))
                .unwrap(),
        )
        .font(font_bytes)
        .unwrap()
        .text_color(Color::from_rgba(180, 180, 180, 255))
        .font_size(40)
        .build();

    let checkbox_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(checkbox_background_bytes, Some(ImageFormat::Png)).unwrap(),
        )
        .background_hovered(
            Image::from_file_with_format(checkbox_hovered_background_bytes, Some(ImageFormat::Png)).unwrap(),
        )
        .background_clicked(
            Image::from_file_with_format(checkbox_clicked_background_bytes, Some(ImageFormat::Png)).unwrap(),
        )
        .build();

    let editbox_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(editbox_background_bytes, Some(ImageFormat::Png)).unwrap(),
        )
        .background_margin(RectOffset::new(2., 2., 2., 2.))
        .font(font_bytes).unwrap()
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .font_size(25)
        .build();

    let combobox_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(combobox_background_bytes, None).unwrap(),
        )
        .background_margin(RectOffset::new(4., 25., 6., 6.))
        .font(font_bytes).unwrap()
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .color(Color::from_rgba(210, 210, 210, 255))
        .font_size(25)
        .build();

    Skin {
        label_style,
        window_style,
        button_style,
        checkbox_style,
        editbox_style,
        combobox_style,
        ..root_ui().default_skin()
    }
}
