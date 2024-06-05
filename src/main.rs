use druid::widget::{Container, Controller, Label, Painter};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, PlatformError, Point, RenderContext,
    RoundedRectRadii, Widget, WidgetExt, WindowDesc, WindowLevel,
};
use rand::Rng;

const TEXT_SIZE: f64 = 20.0;
const W_WIDTH: f64 = TEXT_SIZE + 20.;
const W_HEIGHT: f64 = TEXT_SIZE + 20.;

#[derive(Clone, Data, Lens)]
struct AppState {
    dragging: bool,
    offset: Point,
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_root_widget())
        .title(LocalizedString::new("Random Char Overlay"))
        .window_size((W_WIDTH, W_HEIGHT))
        .resizable(false)
        .transparent(true)
        .show_titlebar(false)
        .set_level(WindowLevel::Tooltip(druid::WindowHandle::default()));

    let launcher = AppLauncher::with_window(main_window);
    launcher.launch(AppState {
        dragging: false,
        offset: Point::ZERO,
    })?;
    Ok(())
}

fn build_root_widget() -> impl Widget<AppState> {
    let label = Label::new(generate_random_char().to_string())
        .with_text_size(TEXT_SIZE)
        .center()
        .background(Painter::new(|ctx, _env, _data| {
            let rect = ctx.size().to_rect();
            ctx.fill(rect, &Color::BLACK);
        }))
        .rounded(RoundedRectRadii::from_single_radius(10.0));

    let main_widget = label.padding(0.0).center().border(Color::TRANSPARENT, 0.);

    main_widget.controller(AppController)
}

struct AppController;

impl Controller<AppState, Container<AppState>> for AppController {
    fn event(
        &mut self,
        child: &mut Container<AppState>,
        ctx: &mut druid::EventCtx<'_, '_>,
        event: &druid::Event,
        data: &mut AppState,
        env: &druid::Env,
    ) {
        match event {
            druid::Event::MouseDown(mouse_event) => {
                if mouse_event.button == druid::MouseButton::Left {
                    data.dragging = true;
                    data.offset = mouse_event.pos;
                    ctx.set_active(true);
                }
            }
            druid::Event::MouseMove(mouse_event) => {
                if data.dragging {
                    let new_pos = ctx.window().get_position().to_vec2() + mouse_event.pos.to_vec2() - data.offset.to_vec2();
                    ctx.window().set_position(Point::new(new_pos.x, new_pos.y));
                }
            }
            druid::Event::MouseUp(mouse_event) => {
                if mouse_event.button == druid::MouseButton::Left && data.dragging {
                    data.dragging = false;
                    ctx.set_active(false);
                }
            }
            _ => (),
        }

        child.event(ctx, event, data, env)
    }
}

fn generate_random_char() -> char {
    let mut rng = rand::thread_rng();
    let ascii_range = 33..=126;
    rng.gen_range(ascii_range) as u8 as char
}
