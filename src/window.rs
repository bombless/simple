
use std;

extern crate sdl2;
extern crate sdl2_image;
use sdl2::render;
use sdl2::video;
use sdl2_image::LoadTexture;

use event::{self,Event};
use shape;

///
/// A Window can display graphics, play sounds, and handle events.
///
/// Creating multiple Windows is untested!
///
pub struct Window {
    // sdl graphics
    context:                    sdl2::sdl::Sdl,
    renderer:                   render::Renderer,

    // events and event logic
    running:                    bool,
    event_queue:                std::vec::Vec<Event>,

    // timing
    target_ticks_per_frame:     u32,
    ticks_at_previous_frame:    u32,
}

/// Top-level Running / Creation Methods
/// ------------------------------------
impl Window {
    /// Intialize a new running window. `name` is used as a caption.
    pub fn new(name: &str, width: i32, height: i32) -> Self {
        // SDL2 Initialization calls. This section here is the reason we can't easily create
        // multiple Windows. There would have to be some kind of global variable that tracked
        // whether SDL2 had already been init'd.
        //
        // Note that initialization is not the only problem. SDL2 is usually safe to init
        // multiple times, but it's not safe to de-init SDL2 and then continue using it. We'd
        // either have to have an explicit Deinitialize() global function or keep a global count
        // of windows that exist.
        //
        // Both solutions are ugly and error-prone, and would probably break thread safety. Going
        // to assume that there will only be one Window per program.
        //
        // TODO: solve this problem
        //
        let sdl_context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();
        sdl2_image::init(sdl2_image::InitFlag::all());
        let sdl_window = video::Window::new(
            name,
            video::WindowPos::PosUndefined,
            video::WindowPos::PosUndefined,
            width, height,
            video::SHOWN,
        ).unwrap();

        let renderer = render::Renderer::from_window(
            sdl_window,
            render::RenderDriverIndex::Auto,
            render::ACCELERATED,
        ).unwrap();

        let window = Window{
            context:                    sdl_context,
            renderer:                   renderer,
            running:                    true,
            event_queue:                vec![],
            target_ticks_per_frame:     (1000.0 / 60.0) as u32,
            ticks_at_previous_frame:    0,
        };
        window.clear();
        window
    }

    /// Redrawing and update the display, while maintaining a consistent framerate and updating the
    /// event queue. You should draw your objects immediately before you call this function. NOTE:
    /// This function returns false if the program should terminate.
    pub fn next_frame(&mut self) -> bool {
        if !self.running {
            return false;
        }

        self.renderer.drawer().present();

        let mut current_ticks = sdl2::timer::get_ticks();
        while current_ticks - self.ticks_at_previous_frame < self.target_ticks_per_frame {
            sdl2::timer::delay(5);
            current_ticks = sdl2::timer::get_ticks();
        }
        self.ticks_at_previous_frame = current_ticks;

        // Handle events
        loop {
            let sdl_event = self.context.event_pump().poll_event();
            match sdl_event {
                None => break,
                Some(sdl_event) => match Event::from_sdl2_event(sdl_event) {
                    Some(Event::Quit) => self.quit(),
                    Some(Event::Keyboard{key: event::KeyCode::Escape, ..})  => self.quit(),

                    // any other unrecognized event
                    Some(e) => (self.event_queue.push(e)),
                    None => (),
                },
            };
        }

        true
    }

    /// Return true when there is an event waiting in the queue for processing.
    pub fn has_event(&self) -> bool { self.event_queue.len() > 0 }

    /// Get the next event from the queue. NOTE: If the event queue on the Window is empty, this
    /// function will panic. Call `has_event()` to find out if there is an event ready for
    /// processing.
    ///
    /// Note that events are handled in a first-in-first-out order. If a user presses three keys 1,
    /// 2, 3 during a frame, then the next three calls to next_event will return 1, 2, 3 in the
    /// same order.
    pub fn next_event(&mut self) -> Event { self.event_queue.remove(0) }

    /// This does not actually cause the program to exit. It just means that next_frame will return
    /// false on the next call.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

/// Drawing Methods
/// ---------------
impl Window {
    /// Windows have a color set on them at all times. This color is applied to every draw
    /// operation. To "unset" the color, call set_color with (255,255,255,255)
    pub fn set_color(&self, red: u8, green: u8, blue: u8, alpha: u8) {
        let color_struct = sdl2::pixels::Color::RGBA(red, green, blue, alpha);
        self.renderer.drawer().set_draw_color(color_struct);
    }

    // These functions are just aliases onto self.renderer.drawer() as you can see.
    pub fn draw_rect(&self, rect: shape::Rect)      { self.renderer.drawer().draw_rect(rect) }
    pub fn fill_rect(&self, rect: shape::Rect)      { self.renderer.drawer().fill_rect(rect) }
    pub fn draw_point(&self, point: shape::Point)   { self.renderer.drawer().draw_point(point) }

    #[unstable]
    pub fn draw_polygon(&self, polygon: shape::Polygon) {
        self.renderer.drawer().draw_points(&polygon.points[..])
    }

    /// Display the image with its top-left corner at (x, y)
    pub fn draw_image(&self, image: &Image, x: i32, y: i32) {
        self.renderer.drawer().copy(&((*image).texture), Some(shape::Rect{
            x: x,
            y: y,
            w: image.get_width(),
            h: image.get_height(),
        }), None);
    }

    /// Clear the screen to black. This will set the Window's draw color to (0,0,0,255)
    pub fn clear(&self) {
        self.set_color(0, 0, 0, 255);
        self.renderer.drawer().clear();
    }
}

/// Image represents a bitmap that can be drawn on the screen.
pub struct Image<'image> {
    texture:    render::Texture<'image>,
    width:      i32,
    height:     i32,
}

impl<'image> Image<'image> {
    pub fn get_width(&self) -> i32  { self.width }
    pub fn get_height(&self) -> i32 { self.height }
}

/// Creation Methods
/// ----------------
impl Window {
    // Load the image at the path you specify.
    //
    // TODO: work out the ownership issues with load_image and make it public.
    #[allow(unused)]
    fn load_image(&self, filename: Path) -> Result<Image,String> {
        let texture = try!(LoadTexture::load_texture(&(self.renderer), &filename));
        Ok(Image{
            width:      texture.query().width,
            height:     texture.query().height,
            texture:    texture,
        })
    }
}

// Dtor for Window.
impl std::ops::Drop for Window {
    /// Close the window and clean up resources.
    fn drop(&mut self) {
        sdl2_image::quit();
    }
}

