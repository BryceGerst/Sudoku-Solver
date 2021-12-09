//#![windows_subsystem = "windows"]

#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
#[macro_use]
extern crate conrod_winit;
extern crate glium;

mod support;
mod solver;

use conrod_core::{widget, Colorable, Positionable, Widget, Sizeable, Labelable};
use glium::Surface;
use solver::CheckablySquare;
use std::cmp;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
const MAX_SIDE_LENGTH: i32 = 16;
const MIN_SIDE_LENGTH: i32 = 4;

// following function from https://stackoverflow.com/questions/50277050/is-there-a-built-in-function-that-converts-a-number-to-a-string-in-any-base
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}


fn update_square_str(s: String, side_length: i32) -> String {
    if s.len() > 0 {
        let result: Result<i32, _> = i32::from_str_radix(&s, (side_length + 1) as u32);//s.parse();
        return match result {
            Err(_) => "".to_string(),
            Ok(num) => if num >= 1 && num <= side_length {format!("{}", s).to_uppercase()} else {"".to_string()}
        };
    } else {
        return "".to_string();
    }
}

fn fill_solved_values(board_str: &mut Vec<Vec<String>>, side_length: i32, success_str: &mut String) {
    let mut board = vec![vec![(1..=side_length).collect::<Vec<i32>>(); side_length as usize]; side_length as usize];
    for r in 0..side_length {
        for c in 0..side_length {
            if board_str[r as usize][c as usize] != "".to_string() {
                let result: Result<i32, _> = i32::from_str_radix(&board_str[r as usize][c as usize], (side_length + 1) as u32);
                match result {
                    Err(_) => {*success_str = "Unable to solve!".to_string(); return;},
                    Ok(num) => if num >= 1 && num <= side_length {solver::update_board(&mut board, num, r as usize, c as usize, side_length);} else {*success_str = "Unable to solve!".to_string(); return;}
                }
            }
        }
    }
    if solver::solve_board(&mut board, side_length) {
        for r in 0..side_length {
            for c in 0..side_length {
                board_str[r as usize][c as usize] = format_radix(board[r as usize][c as usize][0] as u32, (side_length + 1) as u32).to_uppercase();//board[r as usize][c as usize][0].to_string();
            }
        }
        *success_str = "Solved!".to_string();
    } else {
        *success_str = "Unable to solve!".to_string();
    }
}

fn main() {
    // Build the window.
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let window = glium::glutin::window::WindowBuilder::new()
        .with_title("Sudoku Solver")
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(WIDTH, HEIGHT));
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Generate the widget identifiers.
    widget_ids!(struct Ids { puzzle_board, reset_button, solve_button, success_text, sizeup_button, sizedown_button });
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    const FONT_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(FONT_PATH).unwrap();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    // program variables
    let mut success_str: String = "".to_string();
    let mut side_length: i32 = 9;
    let mut puzzle_strs: Vec<Vec<String>> = vec![vec!["".to_string(); side_length as usize]; side_length as usize];

    // end program variables


    let mut should_update_ui = true;
    event_loop.run(move |event, _, control_flow| {
        // Break from the loop upon `Escape` or closed window.
        match &event {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the loop upon `Escape`.
                glium::glutin::event::WindowEvent::CloseRequested
                | glium::glutin::event::WindowEvent::KeyboardInput {
                    input:
                        glium::glutin::event::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = glium::glutin::event_loop::ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        }

        // Use the `winit` backend feature to convert the winit event to a conrod one.
        if let Some(event) = support::convert_event(&event, &display.gl_window().window()) {
            ui.handle_event(event);
            should_update_ui = true;
        }

        match &event {
            glium::glutin::event::Event::MainEventsCleared => {
                if should_update_ui {
                    should_update_ui = false;

                    // Set the widgets.
                    let ui = &mut ui.set_widgets();

                    /*
                    for result in widget::TextBox::new(&box_text)
                        .mid_top_of(ui.window)
                        //.x_position(conrod::position::Position::Absolute(10.0))
                        //.y_position(conrod::position::Position::Absolute(10.0))
                        .w_h(WIDTH as f64 / 4.0, HEIGHT as f64 / 10.0)
                        .color(conrod_core::color::WHITE)
                        .font_size(32)
                        .set(ids.text, ui)
                    {
                        match result {
                            conrod_core::widget::text_box::Event::Enter => fill_solved_values(&mut puzzle_strs, side_length, &mut box_text),//box_text = "".to_string(),
                            conrod_core::widget::text_box::Event::Update(s) => box_text = s
                        }
                    }
                    */

                    for _click in widget::Button::new()
                        .label("+")
                        .mid_top_of(ui.window)
                        .w_h(WIDTH as f64 / 18.0, HEIGHT as f64 / 14.0)
                        .set(ids.sizeup_button, ui)
                    {
                        success_str = "".to_string();
                        let new_root_length = cmp::min(MAX_SIDE_LENGTH.root(), side_length.root() + 1);
                        side_length = new_root_length * new_root_length;
                        puzzle_strs = vec![vec!["".to_string(); side_length as usize]; side_length as usize];
                    }

                    for _click in widget::Button::new()
                        .label("-")
                        .right_from(ids.sizeup_button, 0.0)
                        .w_h(WIDTH as f64 / 18.0, HEIGHT as f64 / 14.0)
                        .set(ids.sizedown_button, ui)
                    {
                        success_str = "".to_string();
                        let new_root_length = cmp::max(MIN_SIDE_LENGTH.root(), side_length.root() - 1);
                        side_length = new_root_length * new_root_length;
                        puzzle_strs = vec![vec!["".to_string(); side_length as usize]; side_length as usize];
                    }

                    for _click in widget::Button::new()
                        .label("Reset")
                        .down_from(ids.sizeup_button, 0.0)
                        .w_h(WIDTH as f64 / 9.0, HEIGHT as f64 / 14.0)
                        .set(ids.reset_button, ui)
                    {
                        success_str = "".to_string();
                        puzzle_strs = vec![vec!["".to_string(); side_length as usize]; side_length as usize];
                    }

                    for _click in widget::Button::new()
                        .label("Solve")
                        .down_from(ids.reset_button, 0.0)
                        .w_h(WIDTH as f64 / 9.0, HEIGHT as f64 / 14.0)
                        .set(ids.solve_button, ui)
                    {
                        fill_solved_values(&mut puzzle_strs, side_length, &mut success_str);
                    }

                    widget::Text::new(&success_str)
                        .down_from(ids.solve_button, 0.0)
                        .w_h(WIDTH as f64 / 4.5, HEIGHT as f64 / 14.0)
                        .left_justify()
                        .font_size(24)
                        .set(ids.success_text, ui);


                    let mut nums = widget::Matrix::new(side_length as usize, side_length as usize)
                        .mid_bottom_of(ui.window)
                        .w_h(WIDTH as f64 / 2.0, WIDTH as f64 / 2.0)
                        .set(ids.puzzle_board, ui);
                    while let Some(num) = nums.next(ui) {
                        let (r, c) = (num.row, num.col);
                        if r < side_length as usize && c < side_length as usize { // for some reason the number of columns doesn't decrease dynamically, but the number of rows does
                            let square = widget::TextBox::new(&puzzle_strs[r][c]).font_size(32).center_justify();
                            for result in num.set(square, ui) {
                                match result {
                                    conrod_core::widget::text_box::Event::Enter => puzzle_strs[r][c] = "".to_string(),
                                    conrod_core::widget::text_box::Event::Update(s) => puzzle_strs[r][c] = update_square_str(s, side_length)
                                }
                            }
                        }
                        
                    }
                    

                    // end setting widgets

                    // Request redraw if the `Ui` has changed.
                    display.gl_window().window().request_redraw();
                }
            }
            glium::glutin::event::Event::RedrawRequested(_) => {
                // Draw the `Ui` if it has changed.
                let primitives = ui.draw();

                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.2, 0.2, 0.2, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
            _ => {}
        }
    })
}