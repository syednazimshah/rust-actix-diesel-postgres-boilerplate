use rand::{distr::Alphanumeric, Rng};

pub fn generate_auth_token(len: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}