use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
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

    let point3 = point1 + point2;
    let point4 = point1 + 1.0;

    println!("point 1 is: {point1:?}");
}
