use bevy::prelude::*;

#[derive(Resource)]
pub struct Awards {
    points: u32,
}

#[derive(Resource)]
pub struct Fates {
    points: u32,
}

pub struct StatisticsPlugin;

impl Plugin for StatisticsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Awards {
            #[cfg(not(feature = "devtools"))]
            points: 0,
            #[cfg(feature = "devtools")]
            points: 0xffff,
        });

        app.insert_resource(Fates {
            #[cfg(not(feature = "devtools"))]
            points: 0,
            #[cfg(feature = "devtools")]
            points: 0xffff,
        });
    }
}
