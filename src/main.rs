//! The simplest possible example that does something.

use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::Rng;
use ggez::event::{Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_CENTER: (f32, f32) = (WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);

type Velocity = na::Vector2<f32>;
type Location = na::Vector2<f32>;
type Shape = graphics::Mesh;

#[derive(Clone, Debug)]
struct Object {
    loc: Location,
    vel: Velocity,
    shape: Shape,
    b_box: f32,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && self.vel == other.vel
    }
}

fn create_ball(
    ctx: &mut Context,
    point: na::Point2<f32>,
    loc: Location,
    vel: Velocity,
) -> Result<Object, ggez::GameError> {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        point,
        20.0,
        2.0,
        graphics::BLACK,
    )?;
    let ball = Object {
        vel,
        loc,
        shape: circle,
        b_box: 20.0,
    };
    Ok(ball)
}

fn wall_collision(o: &mut Object) {
    if o.loc.y >= (WINDOW_HEIGHT - o.b_box) {
        o.vel.y = o.vel.y * (-1.0);
    }
    if o.loc.y <= (0.0 + o.b_box) {
        o.vel.y = o.vel.y * (-1.0);
    }
    if o.loc.x <= (0.0 + o.b_box) {
        o.vel.x = o.vel.x * (-1.0);
    }
    if o.loc.x >= (WINDOW_WIDTH - o.b_box) {
        o.vel.x = o.vel.x * (-1.0);
    }
}

fn object_collision(objects: &mut Vec<Object>) {
    println!("Collision");
    let mut indices: Vec<(usize, usize)> = Vec::new();
    for i in 0..objects.len() {
        for j in 0..objects.len() {
            let o1 = &objects[i];
            let o2 = &objects[j];
            let distance = o1.loc - o2.loc;
            // println!("${:?}", distance.norm());
            if o1.loc != o2.loc && distance.norm() < o1.b_box + o2.b_box {
                indices.push((i, j));
            }
        }
    }
    println!("${:?}", indices);
    for (i, j) in indices {
        let v1 = objects[i].vel - objects[j].vel;
        let v2 = objects[j].vel - objects[i].vel;
        objects[i].vel = v2.scale(5.0);
        objects[j].vel = v1.scale(5.0);
        println!("${:?}",objects[i].vel);
    }
}

struct MainState {
    objects: Vec<Object>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut rng = rand::thread_rng();
        let mut vec: Vec<Object> = Vec::new();
        for i in 1..10 {
            vec.push(create_ball(
                ctx,
                na::Point2::origin(),
                Location::new(
                    rng.gen::<f32>() * WINDOW_WIDTH,
                    rng.gen::<f32>() * WINDOW_HEIGHT,
                ),
                Velocity::new(5.0, 5.0),
            )?);
        }
        let s = MainState { objects: vec };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // self.pos_x = self.pos_x % 800.0 + 1.0;
        for object in &mut self.objects {
            wall_collision(object);
            object.loc = object.loc + object.vel;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for object in self.objects.iter() {
            graphics::draw(
                ctx,
                &object.shape,
                (na::Point2::new(object.loc.x, object.loc.y),),
            )?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        // for object in &mut self.objects {
        //     object.loc.x = x;
        //     object.loc.y = y;
        // }
    }
    
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        println!("${:?}", keycode);
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
