use std::{collections::{BTreeMap, HashMap, HashSet}, error::Error, fs::read_to_string};

mod point3d;
use point3d::{Point3D, ParsePoint3DError};

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_to_string("input.txt")?;
    let points=parse_content(&content)?;
    Ok(())
}


fn step1(points:&[Point3D])->u128{
    let mut distances: BTreeMap<u32,(Point3D,Point3D)>=BTreeMap::new();
    for first in 0..points.len()-1{
        let first_point=points[first];
        for second in first+1..points.len(){
            let second_point=points[second];
            distances.insert(first_point.dist_other_squared(&second_point),(first_point,second_point));
        }
    }

    let mut circuits: HashMap<Point3D,u32>=HashMap::new();
    let mut circuit_number=0_u32;

    for (first_point,second_point) in distances.values().take(1000){
        match(circuits.get_mut(first_point),circuits.get_mut(second_point)){
            (Some(first_circuit),Some(second_circuit))=>{
                let lower_value=(*first_circuit).min(*second_circuit);
                let upper_value=(*first_circuit).max(*second_circuit);
                if lower_value!=upper_value{
                    *second_circuit=lower_value;
                    for (_,value) in circuits.iter_mut(){
                        if *value == upper_value{
                            *value=lower_value;
                        }
                    }
                }
            }
            (None,Some(circuit))=>{
                circuits.insert(*first_point, *circuit);
            }
            (Some(circuit),None)=>{
                circuits.insert(*second_point, *circuit);
            }
            (None,None)=>{
                let circuit=circuit_number;
                circuit_number+=1;
                circuits.insert(*first_point, circuit);
                circuits.insert(*second_point, circuit);
            }
        }
    }


    0
}
struct Circuit{
    id:u32,points:Vec<Point3D>
}



#[derive(Debug)]
struct ParseContentError{
    value: String,error: ParsePoint3DError
}

impl std::fmt::Display for ParseContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse content at '{}': {}", self.value, self.error)
    }
}

impl Error for ParseContentError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}


fn parse_content(content:&str)-> Result<Vec<Point3D>,ParseContentError>{
    content
        .lines()
        .map(|line|line
            .parse::<Point3D>()
            .map_err(|err|ParseContentError{value:line.to_string(),error:err}))
            .collect()
}
