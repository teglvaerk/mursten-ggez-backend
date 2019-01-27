extern crate ggez;
extern crate mursten;
extern crate nalgebra;

use mursten::{Scene, Backend};
use mursten::logic::{Update, ElapsedDelta};
use mursten::graphics::{Draw, Graphics, DrawPrimitives, DrawMode, Color};
use mursten::input::{JoystickProvider, Joystick, JoystickId, Button, Dpad};
use nalgebra::*;


pub struct GgezBackend {
    context: ggez::Context,
    keyboard_joystick: Joystick,
    default_font: ggez::graphics::Font,
}

impl GgezBackend {
    pub fn new(w: u32, h: u32) -> Self {
        let mut c = ggez::conf::Conf::new();
        c.window_setup.title = "t".to_string();
        c.window_mode.width = w;
        c.window_mode.height = h;

        Self {
            context: ggez::Context::load_from_conf("_", "_", c).unwrap(),
            keyboard_joystick: Joystick::default(),
            default_font: ggez::graphics::Font::default_font().unwrap(),
        }
    }
}

impl<Scn> Backend<Scn> for GgezBackend
where
    Scn: Scene + Draw<Screen> + Update<Context>,
{
    fn run(
        mut self, 
        mut scene: Scn
    ) -> Scn {
        let mut events = ggez::event::Events::new(&mut self.context).unwrap();
        let mut continuing = true;

        while continuing {
            // Handle events
            for event in events.poll() {
                self.context.process_event(&event);
                match event {
                    ggez::event::Event::Quit { .. } => {
                        println!("Quitting");
                        continuing = false;
                    },
                    ggez::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                        match keycode {
                            ggez::event::Keycode::A => {
                                self.keyboard_joystick.d_pad = Some(Dpad::Left);
                            },
                            ggez::event::Keycode::S => {
                                self.keyboard_joystick.d_pad = Some(Dpad::Bottom);
                            },
                            ggez::event::Keycode::D => {
                                self.keyboard_joystick.d_pad = Some(Dpad::Right);
                            },
                            ggez::event::Keycode::W => {
                                self.keyboard_joystick.d_pad = Some(Dpad::Up);
                            },
                            ggez::event::Keycode::J => {
                                self.keyboard_joystick.a = Button::BeingHeld;
                            },
                            ggez::event::Keycode::K => {
                                self.keyboard_joystick.b = Button::BeingHeld;
                            },
                            x => {}, // println!("Key down: {:?}", x),
                        }
                    },
                    ggez::event::Event::KeyUp { keycode: Some(keycode), .. } => {
                        match keycode {
                            ggez::event::Keycode::Escape => {
                                println!("Quitting");
                                continuing = false;
                            },
                            ggez::event::Keycode::A => {
                                if self.keyboard_joystick.d_pad == Some(Dpad::Left) {
                                    self.keyboard_joystick.d_pad = None;
                                }
                            },
                            ggez::event::Keycode::S => {
                                if self.keyboard_joystick.d_pad == Some(Dpad::Bottom) {
                                    self.keyboard_joystick.d_pad = None;
                                }
                            },
                            ggez::event::Keycode::D => {
                                if self.keyboard_joystick.d_pad == Some(Dpad::Right) {
                                    self.keyboard_joystick.d_pad = None;
                                }
                            },
                            ggez::event::Keycode::W => {
                                if self.keyboard_joystick.d_pad == Some(Dpad::Up) {
                                    self.keyboard_joystick.d_pad = None;
                                }
                            },
                            ggez::event::Keycode::J => {
                                self.keyboard_joystick.a = Button::Normal;
                            },
                            ggez::event::Keycode::K => {
                                self.keyboard_joystick.b = Button::Normal;
                            },
                            x => {}, // println!("Key up: {:?}", x),
                        }
                    }
                    x => {}, // println!("Event fired: {:?}", x),
                }
            }
            
            // Tell the timer stuff a frame has happened.
            // Without this the FPS timer functions and such won't work.
            self.context.timer_context.tick();
            {
                let mut context = Context { context: self.context, keyboard_joystick: self.keyboard_joystick.clone() };
                scene.update(&mut context);
                self.context = context.into_inner()
            }

            // Draw everything
            {
                let mut screen = Screen { context: self.context, default_font: self.default_font.clone() };
                scene.draw(&mut screen);
                self.context = screen.into_inner()
            }

            ggez::timer::yield_now();
        }
        
        scene
    }
    
    fn quit(&mut self) {}
}

pub struct Context { context: ggez::Context, keyboard_joystick: Joystick }

impl Context {
    fn into_inner(self) -> ggez::Context { self.context }
}

impl ElapsedDelta for Context {
    fn delta(&self) -> f32 {
        let t = ggez::timer::get_delta(&self.context);
        t.as_secs() as f32 + (t.subsec_millis() as f32 / 1000.0)
    }
}

impl JoystickProvider for Context {
    fn joystick(&self, id: JoystickId) -> Joystick {
        if id == 0 {
            self.keyboard_joystick.clone()
        }
        else {
            let id = id - 1;
            
            let gp = ggez::input::get_gamepad(&self.context, id as i32).unwrap();
            let mut j = Joystick::default();
            j.a = gp.button(ggez::event::Button::A).into();
            j.b = gp.button(ggez::event::Button::B).into();
            j.x = gp.button(ggez::event::Button::X).into();
            j.y = gp.button(ggez::event::Button::Y).into();
            let max_value = 32768.0;
            j.left_axis = Vector2::new(gp.axis(ggez::event::Axis::LeftX) as f32 / max_value, gp.axis(ggez::event::Axis::LeftY) as f32 / max_value);
            j.right_axis = Vector2::new(gp.axis(ggez::event::Axis::RightX) as f32 / max_value, gp.axis(ggez::event::Axis::RightY) as f32 / max_value);

            j
        }
    }
    fn available_joysticks(&self) -> Vec<JoystickId> {
        vec![0].iter().cloned().chain(
            (0..4).filter(|id| {
                ggez::input::get_gamepad(&self.context, *id).is_some()
            }).map(|id| { (id + 1) as JoystickId })
        ).collect()
    }
}

pub struct Screen { context: ggez::Context, default_font: ggez::graphics::Font }

impl Screen {
    fn precision() -> f32 { 0.5 }
    fn into_inner(self) -> ggez::Context { self.context }
}


impl Graphics for Screen {
    fn clear<C: Color>(&mut self, color: C) {
        let color = ggez::graphics::Color::from(color.into_rgba());
        ggez::graphics::set_background_color(&mut self.context, color);
        ggez::graphics::clear(&mut self.context);
    }
    fn present(&mut self) {
        ggez::graphics::present(&mut self.context);
    }
}

fn convert_draw_mode(dm: DrawMode) -> ggez::graphics::DrawMode {
    match dm {
        DrawMode::Line(w) => ggez::graphics::DrawMode::Line(w),
        DrawMode::Fill => ggez::graphics::DrawMode::Fill,
    }
}

impl DrawPrimitives for Screen {
    fn set_color<C: Color>(&mut self, color: C) {
        let color = ggez::graphics::Color::from(color.into_rgba());
        ggez::graphics::set_color(&mut self.context, color).unwrap();
    }
    fn circle(&mut self, dm: DrawMode, origin: Point2<f32>, radius: f32) {
        ggez::graphics::circle(
            &mut self.context,
            convert_draw_mode(dm),
            ggez::nalgebra::Point2::new(origin.x, origin.y),
            radius,
            Self::precision(),
        ).unwrap();
    }
    fn ellipse(&mut self, dm: DrawMode, origin: Point2<f32>, width: f32, height: f32) {
        ggez::graphics::ellipse(
            &mut self.context,
            convert_draw_mode(dm),
            ggez::nalgebra::Point2::new(origin.x, origin.y),
            width,
            height,
            Self::precision(),
        ).unwrap();
    }
    fn line(&mut self, origin: Point2<f32>, target: Point2<f32>, width: f32) {
        ggez::graphics::line(
            &mut self.context,
            &[
                ggez::nalgebra::Point2::new(origin.x, origin.y),
                ggez::nalgebra::Point2::new(target.x, target.y),
            ],
            width,
        ).unwrap();
    }
    fn polygon(&mut self, mode: DrawMode, points: &Vec<Point2<f32>>) {
        let points : Vec<_> = points.iter().map(|p| { ggez::nalgebra::Point2::new(p.x, p.y) }).collect();
        ggez::graphics::polygon(
            &mut self.context,
            convert_draw_mode(mode),
            &points
        ).unwrap();
    }
    fn text(&mut self, position: Point2<f32>, text: &str) {
        let text = ggez::graphics::Text::new(&mut self.context, text, &self.default_font).unwrap();
        ggez::graphics::draw(&mut self.context, &text, ggez::nalgebra::Point2::new(position.x, position.y), 0.0);
    }
}

