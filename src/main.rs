extern crate sfml;
extern crate specs;

use sfml::graphics::{Color, PrimitiveType, RenderTarget, RenderWindow, Vertex, Drawable, RenderStates};
use sfml::window::{Event, style, Key};

use specs::{Component, DispatcherBuilder, Join, ReadStorage, System, VecStorage, World,
            WriteStorage};

use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug)]
struct Particle {
    pub vertices : [Vertex; 6]
}

impl Drawable for Particle {
    fn draw<'se, 'tex, 'sh, 'shte>(&'se self,
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        target.draw_primitives(&self.vertices, PrimitiveType::Triangles, states);
    }
}

impl Particle {
    pub fn new() -> Self {
        let mut vertices = [Vertex::default(); 6];
        vertices[0] = Vertex::with_pos_color((0.0, 0.0), Color::red());
        vertices[1] = Vertex::with_pos_color((100.0, 0.0), Color::red());
        vertices[2] = Vertex::with_pos_color((0.0, 100.0), Color::red());

        vertices[3] = Vertex::with_pos_color((100.0, 0.0), Color::green());
        vertices[4] = Vertex::with_pos_color((100.0, 100.0), Color::green());
        vertices[5] = Vertex::with_pos_color((0.0, 100.0), Color::green());


        Particle {
            vertices: vertices
        }
    }
}

impl Component for Particle {
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
        Lifetime {
            acc: 0.0,
            inc: 0.03,
            max: 10.0,
        }
    }
}

impl Component for Lifetime {
    type Storage = VecStorage<Self>;
}


struct FadeSystem;

impl<'a> System<'a> for FadeSystem {
    type SystemData = (WriteStorage<'a, Particle>, WriteStorage<'a, Lifetime>);

    fn run(&mut self, (mut particles, mut lifetimes): Self::SystemData) {
        for (particle, lifetime) in (&mut particles, &mut lifetimes).join() {
            let alpha = ((1.0 - (lifetime.acc / lifetime.max)) * std::u8::MAX as f32) as u8;
            for i in 0..particle.vertices.len() {
                particle.vertices[i].color.a = alpha;
            }
            lifetime.acc += lifetime.inc;
        }
    }
}


struct RenderSystem<'a> {
    render_target: Rc<RefCell<&'a mut RenderWindow>>,
}

impl<'a, 'b> System<'a> for RenderSystem<'b> {
    type SystemData = ReadStorage<'a, Particle>;

    fn run(&mut self, particles: Self::SystemData) {
        for particle in particles.join() {
            self.render_target.borrow_mut().draw(particle)
        }
    }
}


fn main() {
    let video_mode = sfml::window::VideoMode::new(800, 600, 32);
    let mut window = RenderWindow::new(video_mode,
                                       "SFML VertexArray accessors Example",
                                       style::DEFAULT,
                                       &Default::default());
    window.set_vertical_sync_enabled(true);
    let render_wrapper = Rc::new(RefCell::new(&mut window));

    let mut world = World::new();

    // components
    world.register::<Particle>();
    world.register::<Lifetime>();

    // entities
    world.create_entity()
        .with(Particle::new())
        .with(Lifetime::new())
        .build();

    // systems
    let rander_system = RenderSystem {render_target: render_wrapper.clone()};
    let mut dispatcher = DispatcherBuilder::new()
        .add_thread_local(FadeSystem)
        .add_thread_local(rander_system)
        .build();

    loop {
        for e in render_wrapper.borrow().events() {
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
        render_wrapper.borrow_mut().clear(&Color::black());

        // update world
        dispatcher.dispatch(&mut world.res);

        // display things on screen
        render_wrapper.borrow_mut().display();
    }
}