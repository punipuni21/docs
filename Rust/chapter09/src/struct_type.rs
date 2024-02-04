pub enum Shape {
    Rectangle {
        height: f64,
        width: f64,
    },
    Triangle {
        height: f64,
        bottom: f64,
    },
    Circle {
        radius: f64,
    },
    Trapezius {
        upper: f64,
        bottom: f64,
        height: f64,
    },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { height, width } => height * width,
            Shape::Triangle { height, bottom } => height * bottom / 2.0,
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Trapezius {
                upper,
                bottom,
                height,
            } => 0.5 * (upper + bottom) * height,
        }
    }
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        match self {
            Self::Rectangle { .. } => "Rectangle",
            Self::Triangle { .. } => "Triangle",
            Self::Circle { .. } => "Circle",
            Self::Trapezius { .. } => "Trapezius",
        }
        .to_string()
    }
}

#[allow(dead_code)]
pub fn use_struct() {
    let rectangle = Shape::Rectangle {
        height: 10.0,
        width: 20.0,
    };
    let triangle = Shape::Triangle {
        height: 10.0,
        bottom: 20.0,
    };
    let circle = Shape::Circle { radius: 10.0 };
    let trapezius = Shape::Trapezius {
        upper: 10.0,
        bottom: 20.0,
        height: 30.0,
    };
    println!("Rectangle area: {}", rectangle.area());
    println!("Triangle area: {}", triangle.area());
    println!("Circle area: {}", circle.area());
    println!("Trapezius area: {}", trapezius.area());
}
