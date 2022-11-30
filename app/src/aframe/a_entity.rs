use yew::{Context, Html, html};

use crate::aframe::{Geometry, Primitive};

pub struct AEntity {
	pub geometry: Option<Geometry>,
}

impl yew::Component for AEntity {
	type Message = ();
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		AEntity { geometry: Some(Geometry { primitive: Primitive::Sphere }) }
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		let geometry = self.geometry.map(|g| g.to_attr_string());
		let html = html! {
			<a-entity
			geometry={geometry}
			position="0 1.6 -0.6"
			scale=".25 .25 .25"
			material="color: cornflowerblue"
			/>
		};
		html
	}
}
