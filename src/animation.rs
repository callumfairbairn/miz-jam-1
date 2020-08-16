use nannou::wgpu::Color;

enum AnimationType {
    ColourChange{
        from: Color,
        to: Color
    },
    OpacityChange{
        from: f64,
        to: f64
    }
}

pub struct AnimationState {
    a_type: AnimationType,

    frame_count: usize,
    end: usize,

    pub current_action: Option<AnimationAction>,
}

impl AnimationState {
    pub fn new_colour_change(from: Color, to: Color, frames: usize) -> Self {
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

    pub fn new_opacity_change(from: f64, to: f64, frames: usize) -> Self {
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
    Colour(Color),
    Opacity(f64)
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
                let weight = (self.frame_count as f64) / (self.end as f64);
                let out_col = blend_colour(from, to, weight);
                Some(AnimationAction::Colour(out_col))
            },
            AnimationType::OpacityChange{from, to} => {
                let weight = (self.frame_count as f64) / (self.end as f64);
                let out = (from * (1.0 - weight)) + (to * weight);
                Some(AnimationAction::Opacity(out))
            }
        };

        false
    }
}

// Blend between a and b. If weight is 0, output is a, if weight is 1, output is b.
fn blend_colour(colour_a: Color, colour_b: Color, weight: f64) -> Color {
    let a_weight = 1.0 - weight;
    let b_weight = weight;

    let r = (colour_a.r * a_weight) + (colour_b.r * b_weight);
    let g = (colour_a.g * a_weight) + (colour_b.g * b_weight);
    let b = (colour_a.b * a_weight) + (colour_b.b * b_weight);
    let a = (colour_a.a * a_weight) + (colour_b.a * b_weight);
    Color {
        r, g, b, a
    }
}