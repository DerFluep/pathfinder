use crate::float2::Float2;
pub struct Line {
    a: Float2,
    b: Float2,
}

#[allow(dead_code)]
impl Line {
    pub fn new(a: Float2, b: Float2) -> Self {
        Self { a, b }
    }

    pub fn get_a(&self) -> Float2 {
        self.a
    }

    pub fn get_b(&self) -> Float2 {
        self.b
    }

    pub fn print(&self) {
        println!("a x: {} y: {}", self.a.get_x(), self.a.get_y());
        println!("b x: {} y: {}", self.b.get_x(), self.b.get_y());
    }

    pub fn get_col_point(&self, target: Line) -> Float2 {
        let a1 = self.b.get_y() - self.a.get_y();
        let b1 = self.a.get_x() - self.b.get_x();
        let c1 = a1 * (self.a.get_x()) + b1 * (self.a.get_y());

        let a2 = target.b.get_y() - target.a.get_y();
        let b2 = target.a.get_x() - target.b.get_x();
        let c2 = a2 * (target.a.get_x()) + b2 * (target.a.get_y());

        let determinant = a1 * b2 - a2 * b1;

        // no intersection found
        if determinant == 0.0 {
            return Float2::new(f64::MAX, f64::MAX);
        } else {
            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;
            return Float2::new(x, y);
        }
    }
}
