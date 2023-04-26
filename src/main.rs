use fltk::{
    app::{self, App},
    enums::{self, Color, ColorDepth, Event, FrameType},
    image::RgbImage,
    prelude::*,
    *,
};
use rand::{random, Rng};
use state::State;

use crate::{circle::Circle, common_structs::RGBColor};

mod common_structs;
mod state;
mod circle;

const WIND_LABEL: &str = "Floating Objects";
// const WIND_WIDTH: i32 = 1820;
const WIND_WIDTH: i32 = 800;
// const WIND_HEIGHT: i32 = 1000;
const WIND_HEIGHT: i32 = 600;
// const MAIN_IMAGE_WIDTH: i32 = 940;
const MAIN_IMAGE_WIDTH: i32 = 512;
// const MAIN_IMAGE_HEIGHT: i32 = 940;
const MAIN_IMAGE_HEIGHT: i32 = 512;
const MAIN_IMAGE_FRAME_THICKNESS: i32 = 4;
const MAIN_IMAGE_X_POS: i32 = 10;
const MAIN_IMAGE_Y_POS: i32 = 10;
const MENU_HEIGHT: i32 = 32;

#[derive(Clone)]
enum Message {
    Quit,
    AddCircleButEv,
    RemoveCircleButEv,
    WBev,
    BBev,
    GBev,
    LGBev,
    MouseDown(i32, i32),
    MouseDrag(i32, i32),
    MouseMove(i32, i32),
    MouseReleased(i32, i32),
    Tick,
}

enum Colour {
    Black,
    Grey,
    LightGrey,
    White,
}


fn main() {
    let mut rng = rand::thread_rng();
    let mut world_state: State = state::State::new(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);

    let application: App = app::App::default();

    let (s, r) = app::channel();

    let mut wind = window::Window::new(0, 0, WIND_WIDTH, WIND_HEIGHT, WIND_LABEL);

    let mut menu = menu::SysMenuBar::default().with_size(wind.width(), MENU_HEIGHT);
    menu.set_frame(enums::FrameType::FlatBox);
    menu.set_color(enums::Color::Light2);

    menu.add_emit(
        "&File/Quit\t",
        enums::Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        s.clone(),
        Message::Quit,
    );

    let mut framing_frame = frame::Frame::default()
        .with_pos(MAIN_IMAGE_X_POS, MAIN_IMAGE_Y_POS + MENU_HEIGHT)
        .with_size(
            MAIN_IMAGE_WIDTH + MAIN_IMAGE_FRAME_THICKNESS * 2,
            MAIN_IMAGE_HEIGHT + MAIN_IMAGE_FRAME_THICKNESS * 2,
        );
    framing_frame.set_frame(FrameType::EngravedBox);

    let mut image_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_X_POS + MAIN_IMAGE_FRAME_THICKNESS,
            MAIN_IMAGE_Y_POS + MAIN_IMAGE_FRAME_THICKNESS + MENU_HEIGHT,
        )
        .with_size(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);

    // this should intercept mouse events
    let mut ghost_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_X_POS + MAIN_IMAGE_FRAME_THICKNESS,
            MAIN_IMAGE_Y_POS + MAIN_IMAGE_FRAME_THICKNESS + MENU_HEIGHT,
        )
        .with_size(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);
    

    let mut b_add_circle = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS,
        200,
        40,
        "Click to add circle",
    );
    b_add_circle.emit(s.clone(), Message::AddCircleButEv);

    let mut b_remove_circle = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 50,
        200,
        40,
        "Click to remove circle",
    );
    b_remove_circle.emit(s.clone(), Message::RemoveCircleButEv);

    let _change_color_title_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
            MENU_HEIGHT + MAIN_IMAGE_Y_POS + 100,
        )
        .with_size(200, 40)
        .with_label("Change background color:");

    let mut b_white = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 150,
        40,
        40,
        "",
    );
    b_white.set_color(Color::White);
    b_white.emit(s.clone(), Message::WBev);

    let mut b_grey = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20 + (40 + 20) * 1,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 150,
        40,
        40,
        "",
    );
    b_grey.set_color(Color::rgb_color(127, 127, 127));
    b_grey.emit(s.clone(), Message::GBev);

    let mut b_light_grey = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20 + (40 + 20) * 2,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 150,
        40,
        40,
        "",
    );
    b_light_grey.set_color(Color::FrameDefault);
    b_light_grey.emit(s.clone(), Message::LGBev);

    let mut b_black = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20 + (40 + 20) * 3,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 150,
        40,
        40,
        "",
    );
    b_black.set_color(Color::Black);
    b_black.emit(s.clone(), Message::BBev);

    /* let mut b_step = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 200,
        200,
        40,
        "Progress one time step",
    );
    b_step.emit(s.clone(), Message::Step);

    let mut b_start = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 250,
        200,
        40,
        "Start animation",
    );
    b_start.emit(s.clone(), Message::Start);

    let mut b_stop = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 300,
        200,
        40,
        "Stop animation",
    );
    b_stop.emit(s.clone(), Message::Stop); */

    wind.end();
    wind.show();

    let callback_sender = s.clone();
    
    let callback = move |handle| {
        callback_sender.send(Message::Tick);
        
        app::repeat_timeout3(0.033, handle);
    };
    

    let ghost_frame_handle_sender = s.clone();
    ghost_frame.handle(move |_, event: Event| {
        match event {
            Event::Push => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                ghost_frame_handle_sender.send(Message::MouseDown(x, y));
                true
            }
            Event::Drag => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                if x >= 0 && x < MAIN_IMAGE_WIDTH && y >= 0 && y < MAIN_IMAGE_HEIGHT {
                    ghost_frame_handle_sender.send(Message::MouseDrag(x, y));
                }
                true
            }
            Event::Released => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                ghost_frame_handle_sender.send(Message::MouseReleased(x, y));
                true
            }
            Event::Move => {
                let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
                let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
                if x >= 0 && x < MAIN_IMAGE_WIDTH && y >= 0 && y < MAIN_IMAGE_HEIGHT {
                    ghost_frame_handle_sender.send(Message::MouseMove(x, y));
                }
                true
            }
            _ => false,
        }
    });

    

    app::add_timeout3(0.033, callback);

    while application.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Quit => {
                    println!("quitting the app...");
                    fltk::app::quit();
                }
                Message::AddCircleButEv => {
                    println!("Adding circle...");

                    world_state.add_circle(Circle::new(
                        String::from("circle"),
                        (MAIN_IMAGE_WIDTH / 2) as f32,
                        (MAIN_IMAGE_HEIGHT / 2) as f32,
                        rng.gen_range(-2.0..2.0),
                        rng.gen_range(-2.0..2.0),
                        rng.gen_range(15.0..25.0),
                        rng.gen_range(3.0..12.0),
                        1.0,
                        RGBColor {
                            r: random(),
                            g: random(),
                            b: random(),
                        },
                        RGBColor {
                            r: random(),
                            g: random(),
                            b: random(),
                        },
                    ));
                }
                Message::RemoveCircleButEv => {
                    println!("Removing circle...");

                    world_state.remove_circle();
                }
                Message::WBev => {
                    println!("Change background to White.");
                    world_state.replace_background(
                        generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::White)
                    );
                }
                Message::LGBev => {
                    println!("Change background to Light Grey.");
                    world_state.replace_background(
                        generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::LightGrey)
                    );
                }
                Message::GBev => {
                    println!("Change background to Grey.");
                    world_state.replace_background(
                        generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::Grey)
                    );
                }
                Message::BBev => {
                    println!("Change background to Black.");
                    world_state.replace_background(
                        generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::Black)
                    );
                }
                Message::Tick => {
                    redraw_image(&mut world_state, &mut image_frame);
                }
                Message::MouseDown(x, y) => {
                    println!("The image was clicked at coordinates x={}, y={}", x, y);

                    world_state.select_circle(x, y);
                }
                Message::MouseDrag(x, y) => {
                    let circle_index: usize = world_state.selected_circle_index;

                    if world_state.has_selected_circle {
                        world_state.circles[circle_index].accelerate_to_position(x as f32, y as f32);
                    }
                }
                Message::MouseMove(x, y) => {
                    // println!("There was Move event at coordinates x={}, y={}", x, y);
                } 
                _ => {
                    println!("yet undefined event");
                }
            };
        }
    }

    application.run().unwrap();
}


fn redraw_image(world_state: &mut State, image_frame: &mut frame::Frame) {
    world_state.progress_one_step();
    let image_data = world_state.get_rendered_view();
    let image = RgbImage::new(
        &image_data.data,
        image_data.width as i32,
        image_data.height as i32,
        ColorDepth::Rgb8,
    )
    .unwrap();
    image_frame.set_image(Some(image));
    image_frame.redraw();
}

fn generate_image_background(width: i32, height: i32, colour: Colour) -> Vec<u8> {
    let num_pix: usize = (width * height) as usize;

    let data_array: Vec<u8>;

    match colour {
        Colour::Black => data_array = vec![0; num_pix * 3],
        Colour::Grey => data_array = vec![127; num_pix * 3],
        Colour::LightGrey => data_array = vec![191; num_pix * 3],
        Colour::White => data_array = vec![255; num_pix * 3],
    }

    return data_array;
}
