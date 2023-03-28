
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[rustfmt::skip]
impl Color {
    pub const RED:Self = Self{r: 1.0, g: 0.0, b: 0.0, a: 1.0,};
    pub const GREEN:Self = Self{r: 0.0, g: 1.0, b: 0.0, a: 1.0,};
    pub const BLUE:Self = Self{r: 0.0, g: 0.0, b: 1.0, a: 1.0,};
    pub const WHITE:Self = Self{r: 1.0, g: 1.0, b: 1.0, a: 1.0,};
    pub const BLACK:Self = Self{r: 0.0, g: 0.0, b: 0.0, a: 1.0,};
    pub const GREY:Self = Self{r: 0.5, g: 0.5, b: 0.5, a: 1.0,};
    pub const TRANSPARENT:Self = Self{r: 0.0, g: 0.0, b: 0.0, a: 0.0,};
}

impl Color {
    #[rustfmt::skip]
    pub fn new(r: f32, g: f32, b: f32)->Result<Self, String>{
        if 
            1.0 < r || r < 0.0 ||
            1.0 < g || g < 0.0 ||
            1.0 < b || b < 0.0
        { Err(format!("color has to be 0, 1 or in between, but got colors r: {r}, g: {g}, b: {b}")) } else 
        { Ok(Color{r, g, b, a: 1.0}) }
    }

    #[rustfmt::skip]
    pub fn new_a(r: f32, g: f32, b: f32, a: f32)->Result<Self, String>{
        if 
            1.0 < r || r < 0.0 ||
            1.0 < g || g < 0.0 ||
            1.0 < b || b < 0.0 ||
            1.0 < a || a < 0.0 
        { Err(format!("color has to be 0, 1 or in between, but got colors r: {r}, g: {g}, b: {b}, a: {a}")) } else 
        { Ok(Color{r, g, b, a}) }
    }
}

impl TryFrom<(f32, f32, f32, f32)> for Color {
    type Error = String;

    #[rustfmt::skip]
    fn try_from(value: (f32, f32, f32, f32)) -> Result<Self, Self::Error> {
        if 
            1.0 < value.0 || value.0 < 0.0 ||
            1.0 < value.1 || value.1 < 0.0 ||
            1.0 < value.2 || value.2 < 0.0 ||
            1.0 < value.3 || value.3 < 0.0
        { Err(format!("color has to be 0, 1 or in between, but got colors {value:?}")) } else 
        { Ok(Color { r: value.0, g: value.1, b: value.2, a: value.3, }) }
    }
}

impl TryFrom<(f32, f32, f32)> for Color {
    type Error = String;

    #[rustfmt::skip]
    fn try_from(value: (f32, f32, f32)) -> Result<Self, Self::Error> {
        if 
            1.0 < value.0 || value.0 < 0.0 ||
            1.0 < value.1 || value.1 < 0.0 ||
            1.0 < value.2 || value.2 < 0.0
        { Err(format!("color has to be 0, 1 or in between, but got colors {value:?}")) } else 
        { Ok(Color { r: value.0, g: value.1, b: value.2, a: 1.0, }) }
    }
}

impl Into<(f32, f32, f32, f32)> for Color{
    fn into(self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }
}

impl Into<(f32, f32, f32)> for Color{
    fn into(self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }
}

impl Default for Color{
    fn default() -> Self {
        Self::GREY
    }
}
