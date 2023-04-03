
use bevy::{
    prelude::{shape::Circle, *},
    transform::commands,
};

use rand::prelude::*;
use bevy_prototype_lyon::prelude::*;

// For each point spawn a shape bundle, color, and stroke maybe

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(startup_sequence)
        .add_system(point_movement)
        .add_system(line_movement)
        .run();
}

pub const POINT_SPEED: f32 = 200.0;
pub const GRAVITY: f32 = 9.8;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct ObjectName(String);

#[derive(Component)]
struct Mass(i32);

// We have a object this object is a entity with the name Car
// The car has a buck of points associated with it that has owners

#[derive(Bundle)]
struct MassPointGroup {
    name: ObjectName,
    direction: Direction
}
#[derive(Component)]
struct Group;

#[derive(Bundle)]
struct PointMassBundle {
    // These are the properties of a point mass
    mass: Mass,
    position: Position,
    direction: Direction,
    shape: ShapeBundle,
    speed: Speed,
    owner: ObjectName,
    color: Fill,
}

#[derive(Component)]
struct Point;

impl MassPointGroup {
    fn new_group(list_of_points: &Vec<Vec2>) -> Vec<PointMassBundle> {
        let mut point_masses = Vec::new();

        for point in list_of_points {
	    // giving the point a center makes the transform.translation glitchy
            let circle = shapes::Circle {
                radius: 6.,
		..default()
            };

            point_masses.push(PointMassBundle {
                mass: Mass(1),
                speed: Speed(0.),
                position: Position(point.clone()),
// random::<f32>(),random::<f32>()
                direction: Direction(Vec2::new(0.,-1.)),
                shape: ShapeBundle {
                    path: GeometryBuilder::build_as(&circle),
		    transform: Transform::from_xyz(point.clone().x, point.clone().y, 0.),
                    ..default()
                },
                // in the future get the name from MassPointgroup
                owner: ObjectName("Square".to_string()),
                color: Fill::color(Color::WHITE),
            })
        }

        point_masses
    }

    fn draw_paths(list_of_points: &Vec<Vec2>) -> ShapeBundle {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(list_of_points[0]);

        for point in list_of_points {
            path_builder.line_to(*point);
        }

        path_builder.close();
        let path = path_builder.build();

        ShapeBundle {
            path,
            transform: Transform::from_xyz(0., 0., 4.),
            ..default()
        }
    }
}


fn line_movement(
    point_query: Query<&Transform, With<Point>>,
    mut line_query: Query<&mut Path, With<Group>>,
    time: Res<Time>
)
{
    let mut path_builder = PathBuilder::new();
    let points  = point_query.iter().collect::<Vec<&Transform>>();
    for &point in points {
	path_builder.line_to(point.translation.truncate());
    }
    path_builder.close();
    if let Ok(mut path ) = line_query.get_single_mut() {
	*path = path_builder.build();
    }
}

fn point_movement(mut point_query: Query<(&mut Transform, &Point, &Direction, &mut Speed)>, time: Res<Time>) {
    
    for (mut transform, point, direction, mut speed) in point_query.iter_mut() {
        let direction3 = Vec3::new(direction.0.x, direction.0.y, 0.);
	speed.0 += GRAVITY;
        transform.translation += direction3.normalize() * speed.0 * time.delta_seconds();
	println!("{}", transform.translation)
    }
}

// Give MassPointgroup a list of 2d vectors for an object
fn startup_sequence(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());


    // let car = vec![
    //     Vec2::new(0., 0.),
    //     Vec2::new(200., 0.),
    //     Vec2::new(200., 30.),
    //     // Vec2::new(0., 0.),
    // ];
    let car = vec![
        Vec2::new(0., 0.),
        Vec2::new(200., 0.),
        Vec2::new(200., 30.),
        Vec2::new(170., 40.),
        Vec2::new(140., 90.),
        Vec2::new(60., 90.),
        Vec2::new(30., 45.),
        Vec2::new(0., 40.),
        Vec2::new(0., 0.),
    ];

    let points = MassPointGroup::new_group(&car);
    let paths = MassPointGroup::draw_paths(&car);

    for point in points {
        commands.spawn((point, Point));
    }

    commands.spawn((
        paths,
        Stroke::new(Color::WHITE, 4.0),
        MassPointGroup {
            name: ObjectName("car".to_string()),
            direction: Direction(Vec2::new(0., -1.)),
        },
	Group
    ));

    // 	    let circle = shapes::Circle {
    // 		radius: 432.,
    // 		center: Vec2::new(32., 0.),
    // 	    };
    // commands.spawn((

    // 		    ShapeBundle {
    // 			path: GeometryBuilder::build_as(&circle),
    // 			    ..default()
    // 		    },
    // 		    // in the future get the name from MassPointgroup
    // 		    Fill::color(Color::RED),

    // 	));
}

// each point is a

// An arry or shape builders is the only way to do this

// let circle = shapes::Circle {
// 	radius: 32.,
// 	center: Ve2::new(200., 21.),
// };

// commands.spawn(
// 	(
// 	    ShapeBundle {
// 		path: GeometryBuilder::build_as(&circle),
// 		..default()
// 	    },
// 	    Fill::color(Color::CYAN),

// 	)

// );
// commands.spawn((
//     ShapeBundle {
//         path,
//         transform: Transform::from_xyz(0., 75., 0.),
//         ..default()
//     },
//     Stroke::new(Color::BLACK, 10.0),
//     Fill::color(Color::RED),
// ));
