extern crate gloo_utils;
extern crate web_sys;

use yew::prelude::*;

pub use app::*;

use crate::aframe::Xyz;

#[derive(Properties, PartialEq)]
pub struct KanjiBoxProps {
	pub text: String,
	pub position: aframe::Xyz,
	#[prop_or_default]
	pub rotation: aframe::Xyz,
}

impl Default for KanjiBoxProps {
	fn default() -> Self {
		KanjiBoxProps {
			text: String::new(),
			position: Xyz::default(),
			rotation: Xyz::default(),
		}
	}
}

#[function_component(KanjiBox)]
pub fn kanji_block(props: &KanjiBoxProps) -> Html {
	let text_value = props.text.clone();
	let html = html! {
		<a-entity position={props.position} rotation={props.rotation}>
			<a-text value={ text_value } position="0 0 0" side="double" color="black" align="center" shader="msdf" font="https://cdn.glitch.global/78e9163c-0114-44ea-b6d2-3fb627944853/NotoSansJP-Black.json?v=1667671610240" font-image="https://cdn.glitch.global/78e9163c-0114-44ea-b6d2-3fb627944853/NotoSansJP-Black.png?v=1667673945948"></a-text>
			<a-entity class="collidable" position="0 0 0" geometry="primitive: box; width: 1.0; height: 1.0" material="color: #445566; opacity: 0.4; transparent: true"/>
		</a-entity>
	};
	html
}

fn main() {
	yew::start_app::<App>();
}

mod core;
mod app;
pub mod aframe;
