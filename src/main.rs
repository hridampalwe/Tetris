extern crate sdl2;

mod tetris;
mod utils;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use tetris::tetrimino;

const TETRIS_HEIGHT: u32 = 40;

fn handle_events(
    tetris: &mut tetris::Tetris,
    quit: &mut bool,
    timer: &mut SystemTime,
    event_pump: &mut sdl2::EventPump,
) -> bool {
    let mut make_permanent: bool = false;
    if let Some(ref mut piece) = tetris.current_piece {
        let mut tmp_y = piece.y;
        let mut tmp_x = piece.x;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    *quit = true;
                    break;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => {
                    tmp_y += 1;
                    *timer = SystemTime::now();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    tmp_x -= 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    tmp_x += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => {
                    piece.rotate(&tetris.game_map);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let x = piece.x;
                    let mut y = piece.y;
                    while piece.shift(&tetris.game_map, x, y + 1) {
                        y += 1;
                    }
                    make_permanent = true;
                }
                _ => {}
            }
        }
        if make_permanent == false {
            if piece.shift(&tetris.game_map, tmp_x, tmp_y) == false && tmp_y != piece.y {
                make_permanent = true;
            }
        }
    }
    if make_permanent {
        tetris.make_permanent();
        *timer = SystemTime::now();
    }
    make_permanent
}

fn create_rect_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    canvas: &mut Canvas<Window>,
    r: u8,
    g: u8,
    b: u8,
    size_w: u32,
    size_h: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut sq_texture) = texture_creator.create_texture_target(None, size_w, size_h) {
        canvas
            .with_texture_canvas(&mut sq_texture, |texture| {
                texture.set_draw_color(Color::RGB(r, g, b));
                texture.clear();
            })
            .expect("Failed to draw the canvas");
        Some(sq_texture)
    } else {
        None
    }
}

fn print_game_score(tetris: &tetris::Tetris) {
    let mut highscore_change = true;
    let mut nb_lines_change = true;
    if let Some((mut number_of_lines, mut highscore)) = utils::read_high_scores() {
        highscore_change = utils::update_vec(&mut highscore, tetris.score);
        nb_lines_change = utils::update_vec(&mut number_of_lines, tetris.nb_lines);
        if highscore_change || nb_lines_change {
            utils::save_highscore_and_lines(&highscore, &number_of_lines);
        }
    } else {
        utils::save_highscore_and_lines(&[tetris.score], &[tetris.nb_lines]);
    }
    println!(
        "Your score : {}{}",
        tetris.score,
        if highscore_change {
            " [NEW HIGHSCORE]"
        } else {
            ""
        }
    );
    println!(
        "Lines Sent : {}{}",
        tetris.nb_lines,
        if nb_lines_change {
            " [NEW HIGHSCORE]"
        } else {
            ""
        }
    );
    println!("Current Level : {}", tetris.current_level);
}

fn create_texture_from_text<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    text: &str,
    r: u8,
    g: u8,
    b: u8,
) -> Option<Texture<'a>> {
    if let Ok(surface) = font.render(text).blended(Color::RGB(r, g, b)) {
        texture_creator.create_texture_from_surface(&surface).ok()
    } else {
        None
    }
}

fn get_rect_from_text(text: &str, x: i32, y: i32) -> Option<Rect> {
    Some(Rect::new(x, y, text.len() as u32 * 8, 20))
}

fn display_game_information(
    tetris: &tetris::Tetris,
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    start_point: i32,
) {
    let scores_text = format!("Current Score: {}", tetris.score);
    let current_level_text = format!("Current Level: {}", tetris.current_level);
    let lines_sent_text = format!("Lines Sent: {}", tetris.nb_lines);
    let score = create_texture_from_text(&texture_creator, &font, &scores_text, 255, 255, 255)
        .expect("Failed to draw score");
    let current_level =
        create_texture_from_text(&texture_creator, &font, &current_level_text, 255, 255, 255)
            .expect("Failed to draw current_level");
    let lines_sent =
        create_texture_from_text(&texture_creator, &font, &lines_sent_text, 255, 255, 255)
            .expect("Failed to draw lines_sent");
    canvas
        .copy(
            &score,
            None,
            get_rect_from_text(&scores_text, start_point, 90),
        )
        .expect("Failed to copy text_texture");
    canvas
        .copy(
            &current_level,
            None,
            get_rect_from_text(&current_level_text, start_point, 120),
        )
        .expect("Failed to copy text_texture");
    canvas
        .copy(
            &lines_sent,
            None,
            get_rect_from_text(&lines_sent_text, start_point, 150),
        )
        .expect("Failed to copy text_texture");
}

fn display_game_name(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    start_point: i32,
) {
    let text = "Tetris";
    let name = create_texture_from_text(texture_creator, font, text, 255, 255, 255)
        .expect("Failed to draw name");
    canvas
        .copy(
            &name,
            None,
            Rect::new(start_point, 20, text.len() as u32 * 20, 48),
        )
        .expect("Failed to copy the name");
}

fn main() {
    let sdl2_context = sdl2::init().expect("Initialization Failed");
    let video_screen = sdl2_context.video().expect("Failed to gather video screen");
    let height = 800;
    let width = 600;
    let sdl2_text_content = sdl2::ttf::init().expect("Failed to text font content");
    let font = sdl2_text_content
        .load_font("assets/font.ttf", 16)
        .expect("Failed to load the font");
    let font2 = sdl2_text_content
        .load_font("assets/font.ttf", 48)
        .expect("Failed to load the font");
    let window = video_screen
        .window("First Screen", width, height)
        .opengl()
        .build()
        .expect("Failed to create the windows");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Cannot build the canvas");

    let grid_x = 20;
    let grid_y = (height - TETRIS_HEIGHT as u32 * 16 + 40) as i32 / 2;
    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let grid = create_rect_texture(
        &texture_creator,
        &mut canvas,
        50,
        50,
        50,
        TETRIS_HEIGHT as u32 * 10,
        TETRIS_HEIGHT as u32 * 16,
    )
    .expect("Failed to create grid");

    let border = create_rect_texture(
        &texture_creator,
        &mut canvas,
        150,
        150,
        150,
        TETRIS_HEIGHT as u32 * 10 + 20,
        TETRIS_HEIGHT as u32 * 16 + 20,
    )
    .expect("Failed to create border");
    macro_rules! texture {
        ($r : expr, $g: expr , $b : expr) => {
            create_rect_texture(
                &texture_creator,
                &mut canvas,
                $r,
                $g,
                $b,
                TETRIS_HEIGHT,
                TETRIS_HEIGHT,
            )
            .unwrap()
        };
    }
    let textures = [
        texture!(150, 69, 69),
        texture!(150, 150, 69),
        texture!(150, 50, 37),
        texture!(100, 20, 180),
        texture!(77, 50, 150),
        texture!(39, 100, 150),
        texture!(30, 100, 20),
    ];

    let mut event_pump = sdl2_context
        .event_pump()
        .expect("Failed to create event pump");
    let mut timer = SystemTime::now();
    let mut tetris = tetris::Tetris::new();

    loop {
        let mut make_permanent = false;
        if tetris.is_time_over(&timer) {
            if let Some(ref mut piece) = tetris.current_piece {
                make_permanent = !piece.shift(&tetris.game_map, piece.x, piece.y + 1);
            }
            if make_permanent {
                tetris.make_permanent();
            }
            timer = SystemTime::now();
        }
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();
        canvas
            .copy(
                &border,
                None,
                Rect::new(
                    10,
                    (height - TETRIS_HEIGHT * 16 + 40) as i32 / 2 - 10,
                    TETRIS_HEIGHT as u32 * 10 + 20,
                    TETRIS_HEIGHT as u32 * 16 + 20,
                ),
            )
            .expect("Failed to draw border");

        canvas
            .copy(
                &grid,
                None,
                Rect::new(
                    20,
                    (height - TETRIS_HEIGHT * 16 + 40) as i32 / 2,
                    TETRIS_HEIGHT as u32 * 10,
                    TETRIS_HEIGHT as u32 * 16,
                ),
            )
            .expect("Failed to draw border");
        display_game_information(
            &tetris,
            &mut canvas,
            &texture_creator,
            &font,
            width as i32 - 150,
        );
        display_game_name(&mut canvas, &texture_creator, &font2, width as i32 / 2 - 65);
        if tetris.current_piece.is_none() {
            let piece = tetrimino::create_new_tetrimino();
            if !piece.test_current_position(&tetris.game_map) {
                print_game_score(&tetris);
                break;
            }
            tetris.current_piece = Some(piece);
        }
        let mut quit = false;
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
            // Draw the game
            if let Some(ref mut piece) = tetris.current_piece {
                for (line_nb, line) in piece.states[piece.current_state as usize]
                    .iter()
                    .enumerate()
                {
                    for (case_nb, case) in line.iter().enumerate() {
                        if *case == 0 {
                            continue;
                        }
                        canvas
                            .copy(
                                &textures[*case as usize - 1],
                                None,
                                Rect::new(
                                    grid_x
                                        + (piece.x + case_nb as isize) as i32
                                            * TETRIS_HEIGHT as i32,
                                    grid_y + (piece.y + line_nb) as i32 * TETRIS_HEIGHT as i32,
                                    TETRIS_HEIGHT as u32,
                                    TETRIS_HEIGHT as u32,
                                ),
                            )
                            .expect("Failed to draw the tetris");
                    }
                }
            }
        }
        if quit {
            print_game_score(&tetris);
            break;
        }
        for (line_nb, line) in tetris.game_map.iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue;
                }
                canvas
                    .copy(
                        &textures[*case as usize - 1],
                        None,
                        Rect::new(
                            grid_x + case_nb as i32 * TETRIS_HEIGHT as i32,
                            grid_y + line_nb as i32 * TETRIS_HEIGHT as i32,
                            TETRIS_HEIGHT as u32,
                            TETRIS_HEIGHT as u32,
                        ),
                    )
                    .expect("Couldn't copy the texture");
            }
        }
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
