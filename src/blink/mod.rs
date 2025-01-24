use bevy::prelude::*;

#[derive(Component)]
pub struct Blink {
    pub(crate) blink_rate: f32,
    pub(crate) enabled: bool,
    pub(crate) visibility_when_disabled: Visibility,
}

impl Blink {
    pub fn new(blink_rate: f32, enabled: bool, visibility_when_disabled: Visibility) -> Blink {
        Blink {
            blink_rate,
            enabled,
            visibility_when_disabled,
        }
    }
}

pub struct BlinkPlugin;

impl Plugin for BlinkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_blink);
    }
}

fn handle_blink(time: Res<Time>, mut blink_q: Query<(&mut Visibility, &Blink)>) {
    for (mut visiblity, blink) in blink_q.iter_mut() {
        if blink.enabled == false {
            *visiblity = blink.visibility_when_disabled;
            continue;
        }

        let blink_time = 1. / blink.blink_rate;
        let is_on = (time.elapsed_secs() / blink_time) as u32;
        if is_on % 2 == 0 {
            *visiblity = Visibility::Inherited
        } else {
            *visiblity = Visibility::Hidden
        }
    }
}
