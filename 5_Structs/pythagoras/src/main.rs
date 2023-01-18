#[derive(Debug)]
struct Point2 {
    x: f32,
    y: f32,
}

fn main() {
    let scale = 3.0;
    let point1 = Point2 {
        x: 20.0, 
        y: dbg!(15.0 * scale),
    };

    dbg!(&point1);
    println!("{:?}", point1);
    // println!("Distance of point from origin is {}", length(&point1));
    // println!("Angle of point from origin is {}", angle(&point1));
}

// Borrow the struct - don't own it!
// fn angle(point: &Point2) -> f32 {
//     return (point.y/point.x).atan()
// }
//
// fn length(point: &Point2) -> f32 {
//     return (point.x.powi(2) + point.y.powi(2)).sqrt()
// }
