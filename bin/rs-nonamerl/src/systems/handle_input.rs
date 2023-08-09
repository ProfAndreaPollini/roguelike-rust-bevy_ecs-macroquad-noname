use bevy_ecs::system::ResMut;
use rs_nonamerl_core::prelude::UserInput;

pub fn update_user_input(mut user_input: ResMut<UserInput>) {
    user_input.update();
    // println!("update_user_input: {:?}", user_input);
}
