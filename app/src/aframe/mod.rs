use yew::html::IntoPropValue;
use yew::virtual_dom::AttrValue;

pub use a_box::*;
pub use a_entity::*;
pub use a_scene::*;
pub use geometry::*;

mod a_scene;
mod a_box;
mod a_entity;
mod geometry;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Xyz {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Xyz {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Xyz { x, y, z }
	}
}

impl IntoPropValue<Option<AttrValue>> for Xyz {
	fn into_prop_value(self) -> Option<AttrValue> {
		let s = format!("{} {} {}", self.x, self.y, self.z);
		let attr_value = AttrValue::from(s);
		Some(attr_value)
	}
}
