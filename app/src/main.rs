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
		//<a-box position="-1 0.5 -3" rotation="0 45 0" color="#4CC3D9"></a-box>
		let html = html! {
			<a-scene>
				<a-entity position="-1 0.5 -3" rotation="0 45 0" geometry="primitive: box" material="color: #4CC3D9"/>
				<a-plane position="0 0 -4" rotation="-90 0 0" width="4" height="4" color="#7BC8A4"></a-plane>
				<a-sky color="#ECECEC"></a-sky>
				<a-sphere position="0 1.25 -5" radius="1.25" color="#EF2D5E"></a-sphere>
				<a-cylinder position="1 0.75 -3" radius="0.5" height="1.5" color="#FFC65D"></a-cylinder>
			</a-scene>
		};
		html
	}
}

fn main() {
	yew::start_app::<Model>();
}
