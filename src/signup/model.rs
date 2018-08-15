enum TrainingFormula {
  Unknown,
  Subscription,
  Pack
}

pub struct Interests {
  code: bool,
  game_design: bool,
  level_design: bool,
  sound_design: bool,
  two_dimension: bool,
  three_dimension: bool,
  writting: bool
}

pub struct SignUp {
  username: String,
  email: String,
  training_formula: TrainingFormula,
  training_price: f32,
  interests: Interests
}

impl Interests {
  pub fn new() -> Interests {
    Interests {
      code: false,
      game_design: false,
      level_design: false,
      sound_design: false,
      two_dimension: false,
      three_dimension: false,
      writting: false
    }
  }
}

fn training_formula_to_string(tf: TrainingFormula) -> &'static str {
  match tf {
    TrainingFormula::Unknown => "unknown",
    TrainingFormula::Subscription => "subscription",
    TrainingFormula::Pack => "pack"
  }
}
