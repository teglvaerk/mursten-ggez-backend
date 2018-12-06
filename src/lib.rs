extern crate ggez;
extern crate mursten;
extern crate nalgebra;

use mursten::Scene;
use mursten::graphics::{Draw, Graphics, DrawPrimitives, DrawMode, Color};
use mursten::backend::{Backend, UpdateChain, RenderChain};
use nalgebra::*;


pub struct GgezBackend {
    context: ggez::Context,
}

impl GgezBackend {
    fn new(w: u32, h: u32) -> Self {
        let mut c = ggez::conf::Conf::new();
        c.window_setup.title = "t".to_string();
        c.window_mode.width = w;
        c.window_mode.height = h;

        Self {
            context: ggez::Context::load_from_conf("_", "_", c).unwrap(),
        }
    }
}

pub struct Screen<'a>
{
    ctx: &'a mut ggez::Context,
    precision: f32,
}

impl<'a> Screen<'a> {
    fn new(ctx: &'a mut ggez::Context) -> Screen<'a> {
        Screen {
            ctx,
            precision: 0.5,
        }
    }
}

impl<'a, Scn> Backend<Scn> for GgezBackend
where
    Scn: Scene,
{
    type Context = ggez::Context;
    type Screen = Screen<'static>;

    fn run(
        self, 
        update_chain: UpdateChain<Self::Context, Scn>, 
        render_chain: RenderChain<Self::Screen, Scn>, 
        scene: Scn
    ) -> Scn {
        ggez::event::run(&mut self.context, &mut Main::new(scene, update_chain, render_chain)).unwrap();
        scene
    }
    fn quit(&mut self) {}
}


impl<'a> Graphics for Screen<'a> {
    fn clear<C: Color>(&mut self, color: C) {
        let color = ggez::graphics::Color::from(color.into_rgba());
        ggez::graphics::set_background_color(&mut self.ctx, color);
        ggez::graphics::clear(&mut self.ctx);
    }
    fn present(&mut self) {
        ggez::graphics::present(&mut self.ctx);
    }
}

fn convert_draw_mode(dm: DrawMode) -> ggez::graphics::DrawMode {
    match dm {
        DrawMode::Line(w) => ggez::graphics::DrawMode::Line(w),
        DrawMode::Fill => ggez::graphics::DrawMode::Fill,
    }
}

impl<'a> DrawPrimitives for Screen<'a> {
    fn set_color<C: Color>(&mut self, color: C) {
        let color = ggez::graphics::Color::from(color.into_rgba());
        ggez::graphics::set_color(&mut self.ctx, color);
    }
    fn circle(&mut self, dm: DrawMode, origin: Point2<f32>, radius: f32) {
        ggez::graphics::circle(
            &mut self.ctx,
            convert_draw_mode(dm),
            ggez::nalgebra::Point2::new(origin.x, origin.y),
            radius,
            self.precision,
        ).unwrap();
    }
    fn ellipse(&mut self, dm: DrawMode, origin: Point2<f32>, width: f32, height: f32) {
        ggez::graphics::ellipse(
            &mut self.ctx,
            convert_draw_mode(dm),
            ggez::nalgebra::Point2::new(origin.x, origin.y),
            width,
            height,
            self.precision,
        ).unwrap();
    }
    fn line(&mut self, origin: Point2<f32>, target: Point2<f32>, width: f32) {
        ggez::graphics::line(
            &mut self.ctx,
            &[
                ggez::nalgebra::Point2::new(origin.x, origin.y),
                ggez::nalgebra::Point2::new(target.x, target.y),
            ],
            width,
        ).unwrap();
    }
}


pub struct Main<Ctx, Scr, Scn>
where 
    Scn: Scene,
{
    scene: Scn,
    update_chain: UpdateChain<Ctx, Scn>,
    render_chain: RenderChain<Scr, Scn>,
}

impl<Ctx, Scr, Scn> Main<Ctx, Scr, Scn>
where
    Scn: Scene,
{
    pub fn new(
        scene: Scn,
        update_chain: UpdateChain<Ctx, Scn>,
        render_chain: RenderChain<Scr, Scn>
    ) -> Self {
        Self { scene, update_chain, render_chain }
    }
}

impl<Scn> ggez::event::EventHandler for Main<ggez::Context, for<'a> Screen<'a>, Scn>
where
    Scn: Scene,
{
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        self.update_chain.update(ctx, &mut self.scene);
        Ok(())
    }

    fn draw<'b>(&mut self, ctx: &'b mut ggez::Context) -> ggez::GameResult<()> {
        let mut screen = Screen::new(ctx);
        self.render_chain.render(&mut screen, &mut self.scene);
        Ok(())
    }
}

