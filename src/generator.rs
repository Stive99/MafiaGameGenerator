use crate::role::Role;
use rand::seq::SliceRandom;

pub fn shuffle_roles(roles: &mut [Role]) {
	let mut rng = rand::rng();
	roles.shuffle(&mut rng);
}