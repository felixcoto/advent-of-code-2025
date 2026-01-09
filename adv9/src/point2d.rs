use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq,Hash)]
pub struct Point2D{
    pub x:u32,pub y:u32
}


impl Point2D {
    pub fn dist_other_squared(self: &Point2D, other: &Point2D) -> u128 {
        let dx = self.x.abs_diff(other.x) as u128;
        let dy = self.y.abs_diff(other.y) as u128;
        dx.pow(2) + dy.pow(2)
    }

    pub fn dist_other(self: &Point2D, other: &Point2D)-> f64{
        (self.dist_other_squared(other) as f64).sqrt()
    }

    pub fn area_other(self: &Point2D, other: &Point2D)-> u128{
        self.x.abs_diff(other.x) as u128* self.y.abs_diff(other.y) as u128
    }

    pub fn area_other_tiles(self: &Point2D, other: &Point2D)-> u128{
        (self.x.abs_diff(other.x) as u128 + 1) * (self.y.abs_diff(other.y) as u128 + 1)
    }
    
}
#[derive(PartialEq, Eq, Debug)]
pub enum ParsePoint2DError{
    MissingValue(String),ParseValueError(String,ParseIntError)
}

impl std::fmt::Display for ParsePoint2DError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePoint2DError::MissingValue(s) => write!(f, "Missing value: {}", s),
            ParsePoint2DError::ParseValueError(s, e) => write!(f, "Parse error for {}: {}", s, e),
        }
    }
}

impl Error for ParsePoint2DError{

}

impl FromStr for Point2D{
    type Err = ParsePoint2DError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter= s.split(',');
        

        let x:u32=iter.next()
            .ok_or_else(||ParsePoint2DError::MissingValue("x".to_string()))?
            .parse()
            .map_err(|err| ParsePoint2DError::ParseValueError("x".to_string(), err))?;

        let y:u32=iter.next()
            .ok_or_else(||ParsePoint2DError::MissingValue("y".to_string()))?
            .parse()
            .map_err(|err| ParsePoint2DError::ParseValueError("y".to_string(), err))?;
    
        Ok(Point2D{x:x,y:y})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dist_other_squared() {
        let p1 = Point2D { x: 1, y: 2};
        let p2 = Point2D { x: 4, y: 6 }; // dx=3, dy=4  -> dist=25
        assert_eq!(p1.dist_other_squared(&p2), 25);
        // Test commutativity and potential underflow if u32 subtraction was used
        assert_eq!(p2.dist_other_squared(&p1), 25);
    }
    #[test]
    fn test_dist_other() {
        let p1 = Point2D { x: 1, y: 2};
        let p2 = Point2D { x: 4, y: 6 }; // dx=3, dy=4  -> dist=5
        assert_eq!(p1.dist_other(&p2), 5.0);
        // Test commutativity and potential underflow if u32 subtraction was used
        assert_eq!(p2.dist_other(&p1), 5.0);
    }

    #[test]
    fn test_area_other() {
        let p1 = Point2D { x: 1, y: 2};
        let p2 = Point2D { x: 4, y: 6 }; // dx=3, dy=4  -> dist=5
        assert_eq!(p1.area_other(&p2), 12);
        // Test commutativity and potential underflow if u32 subtraction was used
        assert_eq!(p2.area_other(&p1), 12);
    }

     #[test]
    fn test_area_other_tiles() {
        let p1 = Point2D { x: 1, y: 2};
        let p2 = Point2D { x: 4, y: 6 }; // dx=3, dy=4  -> dist=5
        assert_eq!(p1.area_other_tiles(&p2), 20);
        // Test commutativity and potential underflow if u32 subtraction was used
        assert_eq!(p2.area_other_tiles(&p1), 20);
    }

    #[test]
    fn test_from_str_valid() {
        let s = "10,20";
        let p: Point2D = s.parse().expect("Should parse valid string");
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);

    }

    #[test]
    fn test_from_str_missing_value() {
        let s = "10";
        let err = s.parse::<Point2D>().unwrap_err();
        assert_eq!(err, ParsePoint2DError::MissingValue("y".to_string()));
    }

    #[test]
    fn test_from_str_invalid_int() {
        let s = "10,abc,30";
        let err = s.parse::<Point2D>().unwrap_err();
        match err {
            ParsePoint2DError::ParseValueError(field, _) => assert_eq!(field, "y"),
            _ => panic!("Expected ParseValueError, got {:?}", err),
        }
    }
}
