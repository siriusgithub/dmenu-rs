use overrider::*;

use crate::drw::Drw;

#[override_flag(flag = password)]
impl Drw {
    pub fn format_input(&self) -> String {
	(0..self.input.len()).map(|_| "*").collect()
    }
}
