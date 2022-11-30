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
				<a-entity
					scale=".32 .32 .32"
					position="0 1.6 -0.6"
					geometry="primitive: box;"
					material="color: cornflowerblue; opacity: 0.3; transparent: true;">

					<a-entity
						scale=".8 .8 .8"
						position="0 0 0"
						geometry="primitive: box;"
						material="color: gray"/>

					<a-text
						value="Hello"
						align="center"
						color="white"
						position="0 0 .4"/>

					<a-text
						value="World!"
						align="center"
						color="steelblue"
						position="0 0 -.4"
						rotation="0 180 0"/>

				</a-entity>
				<a-entity laser-controls="hand: left;"/>
				<a-entity laser-controls="hand: right;"/>
				<a-entity environment=""></a-entity>
			</a-scene>
		};
		html
	}
}
