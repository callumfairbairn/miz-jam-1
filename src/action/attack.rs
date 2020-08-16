// Some attack actions we can use.
use crate::{
    animation::AnimationState,
    entity::{
        InstanceState,
        Instance
    }
};

use nannou::wgpu;

pub fn quick_attack(player: &mut InstanceState, mobs: &mut [Instance]) {
    println!("Action triggered! Mobs: {}", mobs.len());

    let player_rect = crate::rect::Rect{
        pos: player.pos,
        size: (1.0, 1.0)
    };

    for mob in mobs.iter_mut() {
        if mob.state.is_active() {
            let mob_rect = crate::rect::Rect{
                pos: (mob.movement.x_pos(), mob.movement.y_pos()),
                size: (1.0, 1.0)
            };
        
            if player_rect.collides_with(&mob_rect) {
                println!("Hit for 1 dmg!");
                mob.state.modify_hp(-1);
                let target_col = wgpu::Color{r: 1.0, g: 1.0, b: 1.0, a: 0.0};
                mob.animations.push_back(AnimationState::new_colour_change(wgpu::Color::WHITE, target_col, 4));
                mob.animations.push_back(AnimationState::new_colour_change(target_col, wgpu::Color::WHITE, 4));
            }
        }
    }
}