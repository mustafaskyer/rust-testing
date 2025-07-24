mod shapes {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        pub fn new_circle(radius: f32) -> Result<Circle, String> {
            if radius > 0.0 {
                Ok(Circle { radius })
            } else {
                Err(String::from("radius must be 1 or greater at least"))
            }
        }

        pub fn new_circle_panic(radius: f32) -> Circle {
            match radius {
                ..=0.0 => panic!("Radius should be positive"),
                _ => Circle { radius },
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::shapes::Circle;

    #[test]
    fn larger_circle_should_contain_smaller() {
        let circle1 = Circle::new(100.00);
        let circle2 = Circle::new(10.00);
        assert_eq!(circle1.contains(&circle2), true, "Failed: ....");
    }

    #[test]
    fn smaller_circle_should_not_contain_larger() {
        let circle1 = Circle::new(100.00);
        let circle2 = Circle::new(10.00);
        assert_eq!(circle2.contains(&circle1), false, "Failed: ...");
    }

    #[test]
    fn should_not_create_new() -> Result<(), String> {
        let circle = Circle::new_circle(-1.0)?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Radius should be positive")]
    fn should_not_create_and_panic() {
        let circle = Circle::new_circle_panic(-1.0);
    }
}
