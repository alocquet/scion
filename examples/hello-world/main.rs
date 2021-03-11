use scion::application::Scion;
use scion::legion::{system, Resources, World};
use scion::utils::time::Time;
use log::{info};
use scion::utils::window::WindowDimensions;


use scion::renderer::{RendererType, ScionRenderer};
use miniquad::Context;

struct T;
impl ScionRenderer for T{
    fn draw(&mut self, _context: &mut Context, _world: &mut World, _resource: &mut Resources) {
        unimplemented!()
    }
}

#[system]
fn time(#[resource] time: &Time) {
    info!("Last frame duration : {:?}", time.delta_duration());
}

#[system]
fn screen(#[resource] screen_dimension: &WindowDimensions) {
    info!("Screen dimension : {:?}", screen_dimension);
}

fn main() {
    Scion::app()
        .with_system(time_system())
        .with_system(screen_system())
        .with_renderer(RendererType::Custom(Box::new(T)))
        .run();
}