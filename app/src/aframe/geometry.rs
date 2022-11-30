pub trait Geometry {
	fn primitive(&self) -> String;
	fn attribute_string(&self) -> String {
		format!("primitive: {}", self.primitive())
	}
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BoxGeometry {
	pub width: f64,
	pub height: f64,
	pub depth: f64,
}

impl Geometry for BoxGeometry {
	fn primitive(&self) -> String {
		"box".to_string()
	}
}

impl BoxGeometry {
	pub fn cube(size: f64) -> Self {
		BoxGeometry { width: size, height: size, depth: size }
	}
}
