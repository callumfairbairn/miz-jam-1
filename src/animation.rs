use nannou::prelude::Srgba;

enum AnimationType {
    ColourChange{
        from: Srgba,
        to: Srgba
    },
    OpacityChange{
        from: f32,
        to: f32
    }
}

pub struct AnimationState {
    a_type: AnimationType,

    frame_count: usize,
    end: usize,

    pub current_action: Option<AnimationAction>,
}

impl AnimationState {
    pub fn new_colour_change(from: Srgba, to: Srgba, frames: usize) -> Self {
        AnimationState {
            a_type: AnimationType::ColourChange{
                from,
                to
            },

            frame_count: 0,
            end: frames,

            current_action: None,
        }
    }

    pub fn new_opacity_change(from: f32, to: f32, frames: usize) -> Self {
        AnimationState {
            a_type: AnimationType::OpacityChange{
                from,
                to
            },

            frame_count: 0,
            end: frames,
            
            current_action: None,
        }
    }
}

// The result of an animation tick.
// Tells "view" what to draw.
pub enum AnimationAction {
    Colour(Srgba),
    Opacity(f32)
}

impl AnimationState {
    // Returns true if the animation is done.
    pub fn tick(&mut self) -> bool {
        self.frame_count += 1;
        if self.frame_count > self.end {
            return true;
        }

        self.current_action = match self.a_type {
            AnimationType::ColourChange{from, to} => {
                let weight = (self.frame_count as f32) / (self.end as f32);
                let out_col = blend_colour(from, to, weight);
                Some(AnimationAction::Colour(out_col))
            },
            AnimationType::OpacityChange{from, to} => {
                let weight = (self.frame_count as f32) / (self.end as f32);
                let out = (from * (1.0 - weight)) + (to * weight);
                Some(AnimationAction::Opacity(out))
            }
        };

        false
    }
}

// Blend between a and b. If weight is 0, output is a, if weight is 1, output is b.
fn blend_colour(colour_a: Srgba, colour_b: Srgba, weight: f32) -> Srgba {
    let a_components = colour_a.into_components();
    let b_components = colour_b.into_components();

    let a_weight = 1.0 - weight;
    let b_weight = weight;

    let r = (a_components.0 * a_weight) + (b_components.0 * b_weight);
    let g = (a_components.1 * a_weight) + (b_components.1 * b_weight);
    let b = (a_components.2 * a_weight) + (b_components.2 * b_weight);
    let a = (a_components.3 * a_weight) + (b_components.3 * b_weight);
    Srgba::from_components((r, g, b, a))
}