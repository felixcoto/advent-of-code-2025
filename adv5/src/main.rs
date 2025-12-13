use std::{ collections::HashSet, error::Error};

mod range_set;

fn main() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("input.txt")?;
    let input = parse_content(&content)?;
    println!("step 1: {}",step1(&input));
    println!("step 2: {}",step2(&input)?);
    Ok(())

}


fn step1(input:&Input)->i128{
    let mut result=0;
    for &value in input.to_check.iter(){
        for range in input.database.iter(){
            if range.contains(value){
                result+=1;
                break;
            }
        }
    }

    result
}

fn step2(input:&Input)->Result<i128,Box<dyn Error>>{
    let mut result=0;
    let mut partial_result: HashSet<Range>=HashSet::new();
    for range in input.database.iter(){
        let intersections:Vec<Range>=partial_result.iter().filter(|&other| range.intersects(other)).cloned().collect();
        if intersections.is_empty(){
            partial_result.insert(range.clone());
        }else{
            let mut merged=range.clone();
            for other in intersections.iter(){
                merged=merged.merge(other)?;
                partial_result.remove(other);
            }
            partial_result.insert(merged);
        }
    }

    for range in partial_result{
        result+=range.end-range.start+1;
    }


    Ok(result)
}

fn parse_content(content:&str) -> Result<Input,Box<dyn Error>>{
    let mut database=Vec::new();
    let mut to_check=Vec::new();
    let mut database_finish=false;
    for line in content.lines(){
        if !database_finish{
            if line.eq(""){
                database_finish=true;
            }else {
                database.push(parse_range_line(&line)?);
            }
        }else{
            to_check.push(line.parse::<i128>()?);
        }
    }
    Ok(Input { database: database, to_check:to_check})
}

fn parse_range_line(line:&str)->Result<Range,Box<dyn Error>>{
    if let Some((start_str, end_str))=line.split_once('-'){
        Ok(Range{
            start:start_str.parse()?,end:end_str.parse()?
        })
    }else{
        Err("There is no dash character".into())
    }
}
#[derive(PartialEq, Eq,Debug, Hash,Clone,Copy)]
struct Range{
    start:i128,
    end:i128

}

impl Range {
    pub fn new(start:i128,end:i128)->Range{
        Range{start:start,end:end}
    }
    pub fn contains(self:&Range,number:i128)->bool{
        self.start<=number && self.end>=number
    }
    
    pub fn intersects(self:&Range,other:&Range)->bool{
        !((other.start<self.start && other.end<self.start)||(other.start>self.end && other.end>self.end))
    }

    pub fn merge(self:&Range,other:&Range)->Result<Range,String>{
        if self.intersects(other){
            Ok(Range::new(if self.start<other.start{self.start}else{other.start}, if self.end>other.end{self.end}else{other.end}))
        }else{
            Err(format!("There is no intersection"))
        }
    }
}



struct Input{
    database: Vec<Range>,
    to_check: Vec<i128>
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_intersects(){
        assert!(Range::new(1,5).intersects(&Range::new(1, 5)));
        assert!(Range::new(1,5).intersects(&Range::new(2, 3)));
        assert!(Range::new(2,3).intersects(&Range::new(1, 5)));
        assert!(Range::new(1,5).intersects(&Range::new(2, 6)));
        assert!(Range::new(2,6).intersects(&Range::new(1,5)));
        assert!(!Range::new(2,6).intersects(&Range::new(7,9)));

    }

    #[test]
    

    fn check_merge()->Result<(),Box<dyn Error>>{
        assert_eq!(Range::new(1,5).merge(&Range::new(1, 5))?,Range::new(1,5));
        assert_eq!(Range::new(1,5).merge(&Range::new(2, 3))?,Range::new(1,5));
        assert_eq!(Range::new(2,3).merge(&Range::new(1, 5))?,Range::new(1, 5));
        Ok(())

    }
}