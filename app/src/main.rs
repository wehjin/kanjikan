extern crate gloo_utils;
extern crate web_sys;

use yew::prelude::*;

enum Msg {}

struct Model {}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {}
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		let div = gloo_utils::document().create_element("a-scene").unwrap();
		div.append_child(&aframe::box_element()).unwrap();
		div.append_child(&aframe::sphere_element()).unwrap();
		div.append_child(&aframe::cylinder_element()).unwrap();
		div.append_child(&aframe::plane_element()).unwrap();
		div.append_child(&aframe::sky_element()).unwrap();
		Html::VRef(div.into())
	}
}

fn main() {
	yew::start_app::<Model>();
}

mod aframe {
	use web_sys::Element;

	pub fn box_element() -> Element {
		//<!--a-box position="-1 0.5 -3" rotation="0 45 0" color=" #4CC3D9"></a-box-->
		let el: Element = gloo_utils::document().create_element("a-box").unwrap();
		el.set_attribute("position", "-1 0.5 -3").unwrap();
		el.set_attribute("rotation", "0 45 0").unwrap();
		el.set_attribute("color", "#4CC3D9").unwrap();
		el
	}

	pub fn sphere_element() -> Element {
		//<!--a-sphere position="0 1.25 -5" radius="1.25" color=" #EF2D5E"></a-sphere-->
		let el: Element = gloo_utils::document().create_element("a-sphere").unwrap();
		el.set_attribute("position", "0 1.25 -5").unwrap();
		el.set_attribute("radius", "1.25").unwrap();
		el.set_attribute("color", "#EF2D5E").unwrap();
		el
	}

	pub fn cylinder_element() -> Element {
		//<!--a-cylinder position="1 0.75 -3" radius="0.5" height="1.5" color="#FFC65D"></a-cylinder-->
		let el: Element = gloo_utils::document().create_element("a-cylinder").unwrap();
		el.set_attribute("position", "1 0.75 -3").unwrap();
		el.set_attribute("radius", "0.5").unwrap();
		el.set_attribute("height", "1.5").unwrap();
		el.set_attribute("color", "#FFC65D").unwrap();
		el
	}

	pub fn plane_element() -> Element {
		//<!--a-plane position="0 0 -4" rotation="-90 0 0" width="4" height="4" color=" #7BC8A4"></a-plane-->
		let el: Element = gloo_utils::document().create_element("a-plane").unwrap();
		el.set_attribute("position", "0 0 -4").unwrap();
		el.set_attribute("rotation", "-90 0 0").unwrap();
		el.set_attribute("width", "4").unwrap();
		el.set_attribute("height", "4").unwrap();
		el.set_attribute("color", "#7BC8A4").unwrap();
		el
	}

	pub fn sky_element() -> Element {
		//<!--a-sky color="#ECECEC"></a-sky-->
		let el: Element = gloo_utils::document().create_element("a-sky").unwrap();
		el.set_attribute("color", "#CECECE").unwrap();
		el
	}

// #[function_component]
	// fn a_box() -> Html {
	// 	let el = gloo::utils::document().create_element("a-box").unwrap();
	// }
}