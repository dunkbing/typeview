use druid::widget::{Label, Painter};
use druid::{
    AppLauncher, Color, LocalizedString, PlatformError, RenderContext, RoundedRectRadii, Widget,
    WidgetExt, WindowDesc, WindowLevel,
};
use rand::Rng;

const TEXT_SIZE: f64 = 20.0;
const W_WIDTH: f64 = TEXT_SIZE + 10.0;
const W_HEIGHT: f64 = TEXT_SIZE + 5.0;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_root_widget())
        .title(LocalizedString::new("Random Char Overlay"))
        .window_size((W_WIDTH, W_HEIGHT))
        .resizable(false)
        .transparent(true)
        .show_titlebar(false)
        .set_level(WindowLevel::Tooltip(druid::WindowHandle::default()));

    let launcher = AppLauncher::with_window(main_window);
    launcher.launch(())?;
    Ok(())
}

fn build_root_widget() -> impl Widget<()> {
    let label = Label::new(generate_random_char().to_string())
        .with_text_size(TEXT_SIZE)
        .center()
        .background(Painter::new(|ctx, _env, _data| {
            let rect = ctx.size().to_rect();
            ctx.fill(rect, &Color::BLACK);
        }))
        .rounded(RoundedRectRadii::from_single_radius(10.0));

    let main_widget = label.padding(0.0).center().border(Color::TRANSPARENT, 0.);

    main_widget
}

fn generate_random_char() -> char {
    let mut rng = rand::thread_rng();
    let ascii_range = 33..=126;
    rng.gen_range(ascii_range) as u8 as char
}
