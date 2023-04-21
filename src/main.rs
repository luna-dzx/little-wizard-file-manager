use notan::draw::*;
use notan::prelude::*;
use std::ops::Deref;

mod button;
use button::*;

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .transparent(true)
        .decorations(true)
        .resizable(true);

    notan::init_with(State::new)
        .add_config(win)
        .add_config(DrawConfig)
        .update(update)// Simple way to add the draw extension
        .draw(draw)
        .build()
}

#[derive(AppState)]
pub struct State {
    time: f32,
    button_handler: ButtonHandler,
}

impl State {
    fn new(_gfx: &mut Graphics) -> Self {
        let button_style = ButtonStyle {
            base_color: Color::new(0.5,0.5,0.5,1.0),
            hover_color: Color::new(0.8,0.8,0.8,1.0),
            click_color: Color::new(1.0,1.0,1.0,1.0),
            corner_radius: 5.0,
        };

        let button_handler = ButtonHandler::new(button_style);

        let mut state = Self {
            time: 0.0,
            button_handler,
        };

        state.button_handler.add(
            State::test_state,
            Bounds::new((100.0,100.0),(100.0,50.0))
        );
        state.button_handler.add(
            |_| println!("AAAAAAAAAAA\nwriting cool little closure"),
            Bounds::new((100.0,300.0),(100.0,50.0))
        );


        state
    }

    fn process_buttons(&mut self, app: &mut App){
        let pos = app.mouse.position();
        let clicked = app.mouse.left_is_down();

        let func = self.button_handler.update(pos,clicked);
        (func)(self);
    }

    fn draw_buttons(&mut self, draw: &mut Draw)
    {
        self.button_handler.draw(draw);
    }

    pub fn test_state(state: &mut State){
        println!("woah wahst aaaaa {}",state.time);
    }
}


fn update(app: &mut App, state: &mut State) {
    state.time += app.timer.delta_f32();
    state.process_buttons(app);
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::TRANSPARENT);
    let size = gfx.size();
    draw.triangle((0.5, 0.0), (0.0, 1.0), (1.0, 1.0))
        .scale(size.0 as f32,size.1 as f32)
        .color(Color::MAGENTA);

    state.draw_buttons(&mut draw);

    gfx.render(&draw);
}
