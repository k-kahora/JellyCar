use bevy::{prelude::{*, shape::Circle}, transform::commands};
use bevy_prototype_lyon::prelude::*;

// For each point spawn a shape bundle, color, and stroke maybe

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(startup_sequence)
        .run();
}

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct ObjectName(String);

#[derive(Component)]
struct Mass(i32);

// We have a object this object is a entity with the name Car
// The car has a buck of points associated with it that has owners

#[derive(Bundle)]
struct MassPointGroup {
    name: ObjectName,
}

#[derive(Bundle)]
struct PointMassBundle {

    // These are the properties of a point mass
    mass: Mass,
    position: Position,
    velocity: Velocity,
    shape: ShapeBundle,
    owner: ObjectName,
    color: Fill,
}

struct PathBundle {
    
}

impl MassPointGroup {

    fn new_group(list_of_points: &Vec<Vec2>) -> Vec<PointMassBundle> {

	let mut point_masses = Vec::new();

	for point in list_of_points {

	    let circle = shapes::Circle {
		radius: 6.,
		center: point.clone()
	    };

	    point_masses.push(
		PointMassBundle {
		    mass: Mass(1),
		    position: Position(point.clone()),
		    velocity: Velocity(Vec2::new(0.,0.)),
		    shape: ShapeBundle {
			path: GeometryBuilder::build_as(&circle),
			    ..default()
		    },
		    // in the future get the name from MassPointgroup
		    owner: ObjectName("Square".to_string()),
		    color: Fill::color(Color::WHITE),
		} 	
	    )
	    
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

fn startup_sequence (
   mut commands: Commands 
)
{
    commands.spawn(Camera2dBundle::default());

    let car = 
	vec![Vec2::new(0.,0.),
	     Vec2::new(200., 0.),
	     Vec2::new(170., 60.),
	     Vec2::new(30., 60.),
	     Vec2::new(0.,0.)];
    
    let points = MassPointGroup::new_group(&car);
    let paths = MassPointGroup::draw_paths(&car);

    for point in points {
	commands.spawn(point);
    }

    commands.spawn((paths,
        Stroke::new(Color::WHITE, 1.0),
		    // Fill::color(Color::WHITE),
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

