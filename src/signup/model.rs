#[derive(Debug, Deserialize)]
pub struct PreSignUp {
  pub username: String,
  pub email: String,
  pub training_formula: String,
  pub training_price: String,
  pub interests: Vec<String>,
}
