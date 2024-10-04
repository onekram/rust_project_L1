struct Point {
    x: f64, 
    y: f64
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn get_x(&self) -> f64 { 
        self.x 
    }

    fn get_y(&self) -> f64 { 
        self.y 
    }

    fn set_x(&mut self, value: f64) { 
        self.x = value;
    }

    fn set_y(&mut self, value: f64) { 
        self.y = value;
    }
}

trait Distance {
    fn get_distance_to(&self, other: &Self) -> f64;
}

impl Distance for Point {
    fn get_distance_to(&self, other: &Self) -> f64 {
        let dx = self.get_x() - other.get_x();
        let dy = self.get_y()- other.get_y();
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(4.0, -3.0);
        assert_eq!(p1.get_distance_to(&p2), 5.0);
    }
}
