use std::{collections::{BTreeMap, HashMap}, error::Error, fs::read_to_string};

mod point3d;
use point3d::{Point3D, ParsePoint3DError};

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_to_string("input.txt")?;
    let points=parse_content(&content)?;
    println!("step 1 {}",step1(&points,1000));
    Ok(())
}


fn step1(points:&[Point3D],circuits_to_connect:usize)->u128{
    let mut distances: BTreeMap<u128,(Point3D,Point3D)>=BTreeMap::new();
    for first in 0..points.len()-1{
        let first_point=points[first];
        for second in first+1..points.len(){
            let second_point=points[second];
            distances.insert(first_point.dist_other_squared(&second_point),(first_point,second_point));
        }
    }

    let mut circuits: HashMap<Point3D,u32>=HashMap::new();
    let mut circuit_number=0_u32;

    for (first_point,second_point) in distances.values().take(circuits_to_connect){
        match(circuits.get(first_point).cloned(),circuits.get(second_point).cloned()){
            (Some(first_circuit),Some(second_circuit))=>{
                let lower_value=first_circuit.min(second_circuit);
                let upper_value=first_circuit.max(second_circuit);
                if lower_value!=upper_value{
                    for (_,value) in circuits.iter_mut(){
                        if *value == upper_value{
                            *value=lower_value;
                        }
                    }
                }
            }
            (None,Some(circuit))=>{
                circuits.insert(*first_point, circuit);
            }
            (Some(circuit),None)=>{
                circuits.insert(*second_point, circuit);
            }
            (None,None)=>{
                let circuit=circuit_number;
                circuit_number+=1;
                circuits.insert(*first_point, circuit);
                circuits.insert(*second_point, circuit);
            }
        }
    }

    println!("circuits {:?}",circuits);
    let mut circuit_count=HashMap::new();
    for circuit in circuits.values(){
        *circuit_count
            .entry(*circuit)
            .or_insert(0)+=1_u32;
            
    } 

    let mut circuit_count_vec:Vec<u32>=circuit_count.values().cloned().collect();
    circuit_count_vec.sort();
    circuit_count_vec.reverse();
    println!("circuit_count_vec {:?}",circuit_count_vec);
    circuit_count_vec.iter().take(3).fold(1, |acc,v|(*v) as u128 * acc)
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample(){
        let content = read_to_string("sample.txt").unwrap();
        let points = parse_content(&content).unwrap();
        let result=step1(&points, 10);
        assert_eq!(result,40);
    }
}
