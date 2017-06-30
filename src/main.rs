extern crate sfml;
extern crate specs;
extern crate rand;

use sfml::graphics::{Color, PrimitiveType, RenderTarget, RenderWindow, Vertex, Drawable,
                     RenderStates, Transform as SfmlTransform};
use sfml::window::{Event, style, Key};
use sfml::system::Vector2f;

use specs::{Component, DispatcherBuilder, Join, ReadStorage, System, VecStorage, World,
            WriteStorage};

use rand::distributions::{IndependentSample, Range};

use std::cell::RefCell;
use std::rc::Rc;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const QUAD: f32 = 2.0;
const ENTITIES: usize = 10000;

struct Transform(SfmlTransform);

impl Component for Transform {
    type Storage = VecStorage<Self>;
}


#[derive(Debug)]
struct Quad {
    pub vertices : [Vertex; 6]
}

impl Drawable for Quad {
    fn draw<'se, 'tex, 'sh, 'shte>(&'se self,
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        target.draw_primitives(&self.vertices, PrimitiveType::Triangles, states);
    }
}

impl Quad {
    pub fn new() -> Self {
        let mut quad = Quad {
            vertices: [Vertex::default(); 6]
        };
        quad.rand_pos();
        quad.rand_color();
        quad
    }

    pub fn rand_pos(&mut self) {
        let x_rng = Range::new(0.0, WIDTH as f32 - QUAD);
        let y_rng = Range::new(0.0, HEIGHT as f32 - QUAD);
        let mut rng = rand::thread_rng();
        let (x, y) = (x_rng.ind_sample(&mut rng), y_rng.ind_sample(&mut rng));
        self.vertices[0].position = (x, y).into();
        self.vertices[1].position = (x + QUAD, y).into();
        self.vertices[2].position = (x, y + QUAD).into();
        self.vertices[3].position = (x + QUAD, y).into();
        self.vertices[4].position = (x + QUAD, y + QUAD).into();
        self.vertices[5].position = (x, y + QUAD).into();
    }

    pub fn rand_color(&mut self) {
        let c_rng = Range::new(0u8, std::u8::MAX);
        let mut rng = rand::thread_rng();
        let color = Color::rgba(c_rng.ind_sample(&mut rng), c_rng.ind_sample(&mut rng), c_rng.ind_sample(&mut rng), 1u8);
        for i in 0..self.vertices.len() {
            self.vertices[i].color = color;
        }
    }
}

impl Component for Quad {
    type Storage = VecStorage<Self>;
}


#[derive(Debug)]
struct Lifetime {
    pub acc: f32,
    pub inc: f32,
    pub max: f32,
}

impl Lifetime {
    pub fn new() -> Self {
        let inc_rng = Range::new(0.1, 1.0);
        let max_rng = Range::new(10.0, 100.0);
        let mut rng = rand::thread_rng();
        Lifetime {
            acc: 0.0,
            inc: inc_rng.ind_sample(&mut rng),
            max: max_rng.ind_sample(&mut rng),
        }
    }
}

impl Component for Lifetime {
    type Storage = VecStorage<Self>;
}


struct FadeSystem;

impl<'a> System<'a> for FadeSystem {
    type SystemData = (WriteStorage<'a, Quad>, WriteStorage<'a, Lifetime>);

    fn run(&mut self, (mut particles, mut lifetimes): Self::SystemData) {
        for (particle, lifetime) in (&mut particles, &mut lifetimes).join() {
            let alpha = ((1.0 - (lifetime.acc / lifetime.max)) * std::u8::MAX as f32) as u8;
            for i in 0..particle.vertices.len() {
                particle.vertices[i].color.a = alpha;
            }

            lifetime.acc += lifetime.inc;
            if lifetime.acc >= lifetime.max {
                particle.rand_pos();
                particle.rand_color();
                lifetime.acc = 0.0;
            }
        }
    }
}


struct RenderSystem<'a> {
    render_target: Rc<RefCell<&'a mut RenderWindow>>,
}

impl<'a, 'b> System<'a> for RenderSystem<'b> {
    type SystemData = ReadStorage<'a, Quad>;

    fn run(&mut self, particles: Self::SystemData) {
        for particle in particles.join() {
            self.render_target.borrow_mut().draw(particle)
        }
    }
}


fn main() {
    let video_mode = sfml::window::VideoMode::new(WIDTH, HEIGHT, 32);
    let mut window = RenderWindow::new(video_mode,
                                       "Partycle",
                                       style::DEFAULT,
                                       &Default::default());
    window.set_framerate_limit(60);
    window.set_mouse_cursor_visible(false);
    let window_wrapper = Rc::new(RefCell::new(&mut window));

    let mut world = World::new();

    // components
    world.register::<Transform>();
    world.register::<Quad>();
    world.register::<Lifetime>();

    // entities
    for _ in 0..ENTITIES {
        world.create_entity()
            .with(Quad::new())
            .with(Lifetime::new())
            .build();
    }

    // systems
    let rander_system = RenderSystem {render_target: window_wrapper.clone()};
    let mut dispatcher = DispatcherBuilder::new()
        .add_thread_local(FadeSystem)
        .add_thread_local(rander_system)
        .build();

    loop {
        for e in window_wrapper.borrow().events() {
            match e {
                Event::Closed => return,
                Event::KeyReleased { code: keycode, .. } => match keycode {
                    Key::Escape => return,
                    _ => ()
                },
                _ => ()
            }
        }

        // clear the window
        window_wrapper.borrow_mut().clear(&Color::black());

        // update world
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        // display things on screen
        window_wrapper.borrow_mut().display();
    }
}
