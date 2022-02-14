#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn add_vec(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn add_scalar(self, other: f32) -> Self {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

fn main() {
    let point1 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let point2 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let point3 = point1.add_vec(point2);
    let point4 = point1.add_scalar(1.0);

    println!("point 1 is: {point1:?}");
}
