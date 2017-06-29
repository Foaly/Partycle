extern crate sfml;
extern crate specs;

use sfml::graphics::{Color, PrimitiveType, RenderTarget, RenderWindow, Vertex, Drawable, RenderStates};
use sfml::window::{Event, style};


struct Particle {
    vertices : [Vertex; 6]
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


fn main() {
    let videoMode = sfml::window::VideoMode::new(800, 600, 32);
    let mut window = RenderWindow::new(videoMode,
                                       "SFML VertexArray accessors Example",
                                       style::CLOSE,
                                       &Default::default());
    window.set_vertical_sync_enabled(true);

    let particle = Particle::new();

    loop {
        for e in window.events() {
            if e == Event::Closed {
                return;
            }
            //if e == Event::KeyReleased {
            //    return;
            //}
        }
        // Clear the window
        window.clear(&Color::black());
        window.draw(&particle);
        // Display things on screen
        window.display()

    }
}