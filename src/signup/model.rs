#[derive(Debug, Deserialize)]
pub struct SignUp {
  pub username: String,
  pub email: String,
  pub training_formula: String,
  pub training_price: String,
  pub interest_in_code: Option<String>,
  pub interest_in_game_design: Option<String>,
  pub interest_in_level_design: Option<String>,
  pub interest_in_sound_design: Option<String>,
  pub interest_in_2d: Option<String>,
  pub interest_in_3d: Option<String>,
  pub interest_in_writing: Option<String>
}
