


#[derive(Copy, Clone, Debug)]
pub struct Geometry {
	pub primitive: Primitive,
}

impl Geometry {
	pub fn to_attr_string(&self) -> String {
		format!("primitive: {}", self.primitive.as_tag())
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Primitive {
	Box,
	Sphere,
	Dodecahedron,
	Octahedron,
	Tetrahedron,
}

impl Primitive {
	pub fn as_tag(&self) -> &'static str {
		match self {
			Primitive::Box => "box",
			Primitive::Sphere => "sphere",
			Primitive::Dodecahedron => "dodecahedron",
			Primitive::Octahedron => "octahedron",
			Primitive::Tetrahedron => "tetrahedron",
		}
	}
}
