use yew::{Context, Html, html};

use crate::aframe;

pub struct AScene {}

impl yew::Component for AScene {
	type Message = ();
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		AScene {}
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		let html = html! {
			<a-scene>
				<aframe::ABox/>
				<a-entity environment=""></a-entity>
			</a-scene>
		};
		html
	}
}
