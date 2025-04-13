#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Float2(f64, f64);

impl std::ops::Add for Float2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Float2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Mul<Float2> for Float2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

impl std::ops::Mul<f64> for Float2 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl std::ops::Div<Float2> for Float2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0, self.1 / other.1)
    }
}

impl std::ops::Div<f64> for Float2 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self(self.0 / other, self.1 / other)
    }
}

#[allow(dead_code)]
impl Float2 {
    pub fn new(a: f64, b: f64) -> Self {
        Float2(a, b)
    }
    pub fn get_x(&self) -> f64 {
        self.0
    }
    pub fn get_y(&self) -> f64 {
        self.1
    }
    pub fn set_x(&mut self, x: f64) {
        self.0 = x;
    }
    pub fn set_y(&mut self, y: f64) {
        self.1 = y;
    }
    pub fn length(self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
    pub fn make_unit(mut self) -> Self {
        let length = 1.0 / self.length();
        self.0 *= length;
        self.1 *= length;
        self
    }
    pub fn print(&self) {
        println!("x: {} y: {}", self.0, self.1);
    }
}
