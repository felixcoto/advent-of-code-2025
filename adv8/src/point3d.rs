use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq,Hash)]
pub struct Point3D{
    pub x:u32,pub y:u32,pub z:u32
}


impl Point3D {
    pub fn dist_other_squared(self: &Point3D, other: &Point3D) -> u128 {
        let dx = self.x.abs_diff(other.x) as u128;
        let dy = self.y.abs_diff(other.y) as u128;
        let dz = self.z.abs_diff(other.z) as u128;
        dx.pow(2) + dy.pow(2) + dz.pow(2)
    }
    
}
#[derive(PartialEq, Eq, Debug)]
pub enum ParsePoint3DError{
    MissingValue(String),ParseValueError(String,ParseIntError)
}

impl std::fmt::Display for ParsePoint3DError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePoint3DError::MissingValue(s) => write!(f, "Missing value: {}", s),
            ParsePoint3DError::ParseValueError(s, e) => write!(f, "Parse error for {}: {}", s, e),
        }
    }
}

impl Error for ParsePoint3DError{

}

impl FromStr for Point3D{
    type Err = ParsePoint3DError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter= s.split(',');
        

        let x:u32=iter.next()
            .ok_or_else(||ParsePoint3DError::MissingValue("x".to_string()))?
            .parse()
            .map_err(|err| ParsePoint3DError::ParseValueError("x".to_string(), err))?;

        let y:u32=iter.next()
            .ok_or_else(||ParsePoint3DError::MissingValue("y".to_string()))?
            .parse()
            .map_err(|err| ParsePoint3DError::ParseValueError("y".to_string(), err))?;

        let z:u32=iter.next()
            .ok_or_else(||ParsePoint3DError::MissingValue("z".to_string()))?
            .parse()
            .map_err(|err| ParsePoint3DError::ParseValueError("z".to_string(), err))?;
        
        Ok(Point3D{x:x,y:y,z:z})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dist_other() {
        let p1 = Point3D { x: 1, y: 2, z: 3 };
        let p2 = Point3D { x: 4, y: 6, z: 15 }; // dx=3, dy=4, dz=12 -> dist=13
        assert_eq!(p1.dist_other_squared(&p2), 169);
        // Test commutativity and potential underflow if u32 subtraction was used
        assert_eq!(p2.dist_other_squared(&p1), 169);
    }

    #[test]
    fn test_from_str_valid() {
        let s = "10,20,30";
        let p: Point3D = s.parse().expect("Should parse valid string");
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
        assert_eq!(p.z, 30);
    }

    #[test]
    fn test_from_str_missing_value() {
        let s = "10,20";
        let err = s.parse::<Point3D>().unwrap_err();
        assert_eq!(err, ParsePoint3DError::MissingValue("z".to_string()));
    }

    #[test]
    fn test_from_str_invalid_int() {
        let s = "10,abc,30";
        let err = s.parse::<Point3D>().unwrap_err();
        match err {
            ParsePoint3DError::ParseValueError(field, _) => assert_eq!(field, "y"),
            _ => panic!("Expected ParseValueError, got {:?}", err),
        }
    }
}
