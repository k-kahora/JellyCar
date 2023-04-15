use bevy::{
    prelude::{shape::Circle, *},
    transform::{commands, self},
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
pub const GRAVITY: f32 = 9.;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Point;

#[derive(Component)]
struct Direction(Vec2);

// TODO Derive a speed compontent and use it to make the speed of eack poit independat
#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct ObjectName(String);

#[derive(Component)]
struct Mass(i32);

#[derive(Component)]
struct Force(Vec2);

#[derive(Component)]
struct DampingFactor(i32);

// We have a object this object is a entity with the name Car
// The car has a buck of points associated with it that has owners

#[derive(Bundle)]
struct Spring {
    // Two points
    // Stiffness
    // Rest length
    // Dampang Factor
}
#[derive(Component)]
struct Square{
    points: Vec<Vec2>,
    
}

// We have a BoundingBox that has its own shape
// position
// And stroke color
#[derive(Bundle)]
struct BoundingBoxBundle {
    bounding_square: Square,
    stroke: Stroke,
    // shape: ShapeBundle,
}

impl Default for Square {
    fn default() -> Self {
	Square { points: vec![
	    Vec2::new(0., 0.),
	    Vec2::new(0., 1.),
	    Vec2::new(1., 1.),
	    Vec2::new(1., 0.),
	    Vec2::new(0., 0.),
	    ]
	}
    }
}

#[derive(Bundle)]
struct MassPointGroup {
}

#[derive(Component)]
struct Group;

#[derive(Bundle)]
struct PointMassBundle {
    // These are the properties of a point mass
    mass: Mass,
    position: Position,
    direction: Direction,
    // Later replace speed with force
    speed: Speed,

    // Superflous data
    shape: ShapeBundle,
    color: Fill,
}

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
                position: Position(point.clone()),
// random::<f32>(),random::<f32>()
                direction: Direction(Vec2::new(0., -1.)),
                shape: ShapeBundle {
                    path: GeometryBuilder::build_as(&circle),
		    transform: Transform::from_xyz(point.clone().x, point.clone().y, 0.),
                    ..default()
                },
                // in the future get the name from MassPointgroup
                color: Fill::color(Color::WHITE),
		speed: Speed(0.),
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

// The line is the parent and the points are the children
// Query children in the line query

fn minimum_bounding_box(
    point_query: Query<&Transform, With<Point>>,
    mut line_query: Query<(&mut Path, &Children), With<Group>>,
    time: Res<Time>
)
{

}


// Bounding Box needs to be calculated every frame for all non moving entitys


fn line_movement(
    point_query: Query<&Transform, With<Point>>,
    mut line_query: Query<(&mut Path, &Children), With<Group>>,
    time: Res<Time>
)
{
    for (mut path, children) in line_query.iter_mut() {
	let mut path_builder = PathBuilder::new();
	for &child  in children.iter() {
	    let point = point_query.get(child);
	    if let Ok(transform) = point {
		path_builder.line_to(transform.translation.truncate());
	    }
	}
	path_builder.close();
	*path = path_builder.build();
    }
}

fn point_movement(mut point_query: Query<(&mut Transform, &Point, &Direction, &mut Speed)>, time: Res<Time>) {
    for (mut transform, point, velocity, mut speed) in point_query.iter_mut() {
        let direction = Vec3::new(velocity.0.x, velocity.0.y, 0.);
	speed.0 += GRAVITY;
        transform.translation += direction.normalize() * speed.0 * time.delta_seconds();
    }
}

// Give MassPointgroup a list of 2d vectors for an object
fn startup_sequence(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let trapezoid = vec![
        Vec2::new(-200., 20.),
        Vec2::new(-100., 20.),
        Vec2::new(-100., 120.),
        Vec2::new(-200., 220.),
        Vec2::new(-200., 20.),
    ];

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

    let square_points = MassPointGroup::new_group(&trapezoid);
    let square_lines  = MassPointGroup::draw_paths(&trapezoid);

    commands.spawn((
        paths,
        Stroke::new(Color::WHITE, 4.0),
	Group
    )).with_children(|parent| {
		     for point in points {
			 parent.spawn((point, Point));
		     }
    });

    // Parent is the lines, child is the bounding box, and children are all the points
    commands.spawn((
        square_lines,
        Stroke::new(Color::WHITE, 4.0),
	Group,
    )).with_children(|parent| {
		     for point in square_points {
			 parent.spawn((point, Point));
		     }

    });
}
