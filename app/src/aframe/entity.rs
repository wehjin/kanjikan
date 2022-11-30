use yew::{Context, Html, html};

use crate::aframe::{BoxGeometry, Geometry};

pub struct AEntity {
	pub geometry: Option<Box<dyn Geometry>>,
}

impl yew::Component for AEntity {
	type Message = ();
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		AEntity { geometry: Some(Box::new(BoxGeometry::cube(0.4))) }
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		let geometry = self.geometry.as_ref().map(|g| g.attribute_string());
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
