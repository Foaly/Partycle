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



struct RendererSystem<'a> {
    render_target: Rc<RefCell<&'a mut RenderWindow>>,
}

impl<'a, 'b> System<'a> for RendererSystem<'b> {
    type SystemData = ((), ReadStorage<'a, Particle>);

    fn run(&mut self, (_, particles): Self::SystemData) {
        for particle in (&particles).join() {
            self.render_target.borrow_mut().draw(particle)
        }
    }
}


fn main() {
    let videoMode = sfml::window::VideoMode::new(800, 600, 32);
    let mut window = RenderWindow::new(videoMode,
                                       "SFML VertexArray accessors Example",
                                       style::DEFAULT,
                                       &Default::default());
    window.set_vertical_sync_enabled(true);

    let wrapper = Rc::new(RefCell::new(&mut window));
    let renderer = RendererSystem {render_target: wrapper.clone()};

    let mut world = World::new();
    world.register::<Particle>();


    world.create_entity().with(Particle::new()).build();

    let mut dispatcher = DispatcherBuilder::new()
        .add_thread_local(renderer)
        .build();

    loop {
        for e in wrapper.borrow().events() {
            match e {
                Event::Closed => return,
                Event::KeyReleased { code: keycode, .. } => match keycode {
                    Key::Escape => return,
                    _ => ()
                },
                _ => ()
            }
        }


        // Clear the window
        wrapper.borrow_mut().clear(&Color::black());

        dispatcher.dispatch(&mut world.res);

        //window.draw(&particle);
        // Display things on screen
        wrapper.borrow_mut().display()
    }
}