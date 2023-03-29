use bevy_prototype_lyon::prelude::*;
use bevy::prelude::*;

// Draw a point on the screen

fn main() {

    App::new()
        .insert_resource(Msaa::Sample4)
	.add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
	.add_startup_system(setup_system).run();

}


fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 30,
        feature: shapes::RegularPolygonFeature::Radius(400.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::RED),
        Stroke::new(Color::BLACK, 10.0),
    ));
}
