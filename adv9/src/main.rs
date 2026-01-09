use std::{error::Error, fs};

use crate::point2d::Point2D;

mod point2d;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input.txt")?;
    let points = parse_content(&content)?;
    println!("step1 {:?}",step1(&points));
    Ok(())
}

fn parse_content(content:&str)->Result<Vec<Point2D>,Box<dyn Error>>{
    content.lines().map(|line|line.parse::<Point2D>().map_err(|e|e.into())).collect()
}

fn step1(points:&[Point2D])->Option<u128>{
    let mut result=None;
    if points.len()>=2{
        for first_index in 0..points.len()-1{
            for second_index in first_index+1..points.len(){
                let current_area=points[first_index].area_other_tiles(&points[second_index]);
                result=Some(result.map_or(current_area, |v: u128|v.max(current_area)));
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_step1() -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string("test.txt")?;
        let points = parse_content(&content)?;
        let result=step1(&points);
        assert_eq!(Some(50),result);
        Ok(())
    }
}
