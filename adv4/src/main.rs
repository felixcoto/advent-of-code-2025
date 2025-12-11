use std::{collections::HashMap, error::Error, fs::File, io::Read};

fn main() -> Result<_,Box<dyn Error>>{
    let mut file=File::open("input.txt")?;
    let mut content=String::new();
    file.read_to_string(buf)?;
    let rollPositions=parse_complete_string(content)?;
    
    Ok(())
}


fn step1(roll_positions: &[RollPosition] )->Result<u32,Box<dyn Error>>{
    let mut map=HashMap::new();
    for pos in roll_positions{
        map.insert(pos, 0_u8);
    }
    
    for roll_position in roll_positions{
        for xdiff in [-1,0,1]{
            for ydiff in [-1,0,1]{
                if !(xdiff==0 && ydiff==0){
                    let adj_pos=RollPosition{x:roll_position.x+xdiff,y:roll_position.y+ydiff};
                    if let Some(current_adj) = map.get_mut(&adj_pos){
                        *current_adj=*current_adj+1;
                    }
                }
            }
        }
    }
    Ok(map.values().filter(|count|**count<=4_u8).count() as u32)
}

fn parse_complete_string(content:&str)->Result<Vec<RollPosition>,Box<dyn Error>>{
    let mut result= Vec::new();
    let mut line_number=0;
    for line in content.lines() {
        let mut partial=parse_line(line, line_number)?;
        result.append(&mut partial);
        line_number+=1;
    }
    Ok(result)
}

fn parse_line(line:&str, x:u32) -> Result<Vec<RollPosition>,Box< dyn Error>>{
    let mut result= Vec::new();
    for (index,character) in line.char_indices(){
        if character.eq(&'@'){
            result.push(RollPosition{
                x:x as i32,y:index as i32
            });
        }
    }
    Ok(result)
}
#[derive(PartialEq, Eq, Hash)]
struct RollPosition{
    x:i32,y:i32
}