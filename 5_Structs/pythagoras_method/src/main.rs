struct Point2 {
    x: f32,
    y: f32,
}

impl Point2 {
    fn angle(&self) -> f32 {
        return (self.y/self.x).atan();
    }    
}

impl Point2 {
    fn length(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt()
    }    
}

fn main() {
    let point1 = Point2 {
        x: 20.0,
        y: 15.0,
    };

    println!("The point is {} units from the origin.", point1.length());
    println!("The angle of the point to the origin is {}", point1.angle());
}
