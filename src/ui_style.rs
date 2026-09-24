// ESTILO DE LA INTERFAZ
// let skin1 = {
//     let font = load_ttf_font("/font/press-start-2p-latin-400-normal.ttf")
//     .await
//     .unwrap();
//
//     let label_style = root_ui()
//     .style_builder()
//     .with_font(&font)
//     .unwrap()
//     .text_color(Color::from_rgba(40, 40, 26, 255))
//     .font_size(30)
//     .build();
//
//     let button_style = root_ui()
//     .style_builder()
//     .color(Color::from_rgba(180, 180, 120, 255))
//     .with_font(&font)
//     .unwrap()
//     .build();
//
//     Skin {
//         button_style,
//         label_style,
//         ..root_ui().default_skin()
//     }
// };
//
// let win1 = skin1.clone();
//
// root_ui().push_skin(&win1);
use macroquad::{color::*, text::load_ttf_font, ui::*};

pub async fn ui_skin() -> Skin {
    let font = load_ttf_font(
        "/home/ruls/Documentos/pruebas/naves/src/font/press-start-2p-latin-400-normal.ttf",
    )
    .await
    .unwrap();

    let label_style = root_ui()
        .style_builder()
        .with_font(&font)
        .unwrap()
        .text_color(Color::from_rgba(40, 40, 26, 255))
        .font_size(30)
        .build();

    let button_style = root_ui()
        .style_builder()
        .color(Color::from_rgba(180, 180, 120, 255))
        .with_font(&font)
        .unwrap()
        .build();

    Skin {
        button_style,
        label_style,
        ..root_ui().default_skin()
    }
}
