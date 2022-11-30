#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Prompt {
	Text(String)
}

impl Prompt {
	pub fn text<S: AsRef<str>>(s: S) -> Self {
		let s = s.as_ref().to_string();
		Prompt::Text(s)
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Answer {
	Text(String)
}

impl Answer {
	pub fn text<S: AsRef<str>>(s: S) -> Self {
		let s = s.as_ref().to_string();
		Answer::Text(s)
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Challenge {
	pub prompt: Prompt,
	pub answer: Answer,
}

#[derive(Clone, Debug)]
pub struct GameState {
	challenges: Vec<Challenge>,
	ready: Vec<usize>,
	failed: Vec<usize>,
	passed: Vec<usize>,
	failed_and_passed: Vec<usize>,
}

impl GameState {
	pub fn new(challenges: &Vec<Challenge>) -> Self {
		let ready = (0..challenges.len()).into_iter().collect();
		GameState {
			challenges: challenges.clone(),
			ready,
			failed: Vec::new(),
			passed: Vec::new(),
			failed_and_passed: Vec::new(),
		}
	}
	pub fn next_challenge(&self) -> Option<&Challenge> {
		if self.ready.len() > 0 {
			Some(&self.challenges[self.ready[0]])
		} else if self.failed.len() > 0 {
			Some(&self.challenges[self.failed[0]])
		} else {
			None
		}
	}
	pub fn pass(&self) -> Self {
		let mut new_game = self.clone();
		if self.ready.len() > 0 {
			new_game.passed.push(self.ready[0]);
			new_game.ready.remove(0);
		} else if self.failed.len() > 0 {
			new_game.failed_and_passed.push(self.failed[0]);
			new_game.failed.remove(0);
		}
		new_game
	}
	pub fn fail(&self) -> Self {
		let mut new_game = self.clone();
		if self.ready.len() > 0 {
			new_game.failed.push(self.ready[0]);
			new_game.ready.remove(0);
		} else if self.failed.len() > 0 {
			new_game.failed.push(self.failed[0]);
			new_game.failed.remove(0);
		}
		new_game
	}

	pub fn is_over(&self) -> bool {
		self.ready.is_empty() && self.failed.is_empty()
	}

	pub fn passed_challenges(&self) -> Vec<Challenge> {
		self.passed.iter().map(|i| self.challenges[*i].clone()).collect()
	}
	pub fn failed_and_passed_challenges(&self) -> Vec<Challenge> {
		self.failed_and_passed.iter().map(|i| self.challenges[*i].clone()).collect()
	}

	pub fn reset(&self) -> Self {
		GameState::new(&self.challenges)
	}
}

#[cfg(test)]
mod tests {
	use crate::core::{Answer, Challenge, GameState, Prompt};

	fn challenges() -> Vec<Challenge> {
		let challenges = vec![
			Challenge {
				prompt: Prompt::text("差"),
				answer: Answer::text("さ"),
			}
		];
		challenges
	}

	#[test]
	pub fn init() {
		let challenges = challenges();
		let game = GameState::new(&challenges);
		let next_challenge = game.next_challenge().expect("Challenge exists");
		assert_eq!(next_challenge, &challenges[0]);
	}

	#[test]
	pub fn pass() {
		let challenges = challenges();
		let game = GameState::new(&challenges);
		let game = game.pass();
		assert!(game.is_over());
		assert_eq!(challenges, game.passed_challenges());
		assert_eq!(0, game.failed_and_passed_challenges().len());
	}

	#[test]
	pub fn game_a() {
		let challenges = challenges();
		let game = GameState::new(&challenges);
		let game = game.fail();
		assert!(!game.is_over());
		let game = game.pass();
		assert!(game.is_over());
		assert_eq!(0, game.passed_challenges().len());
		assert_eq!(challenges, game.failed_and_passed_challenges());
		let game = game.reset();
		assert!(!game.is_over());
		assert_eq!(0, game.passed_challenges().len());
		assert_eq!(0, game.failed_and_passed_challenges().len());
	}
}