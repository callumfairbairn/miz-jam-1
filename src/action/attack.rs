// Some attack actions we can use.
use crate::entity::{
    InstanceState,
    Instance
};

pub fn quick_attack(player: &mut InstanceState, mobs: &mut [Instance]) {
    println!("Action triggered! Mobs: {}", mobs.len());

    let player_rect = crate::rect::Rect{
        pos: player.pos,
        size: (32.0, 32.0)
    };

    for mob in mobs.iter_mut() {
        if mob.state.is_active() {
            let mob_rect = crate::rect::Rect{
                pos: (mob.movement.x_pos(), mob.movement.y_pos()),
                size: (32.0, 32.0)
            };
        
            if player_rect.collides_with(&mob_rect) {
                println!("Hit for 1 dmg!");
                mob.state.modify_hp(-1);
            }
        }
    }
}