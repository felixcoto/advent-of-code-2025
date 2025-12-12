use std::{collections::{HashMap, HashSet}, error::Error, fs::File, io::Read};

fn main() -> Result<(),Box<dyn Error>>{
    let mut file=File::open("input.txt")?;
    let mut content=String::new();
    file.read_to_string(&mut content)?;
    let roll_positions=parse_complete_string(&content)?;
    
    println!("step1 result,{}",step1(&roll_positions)?);
    println!("step2 result,{}",step2(&roll_positions)?);
    println!("step2_fast result,{}",step2_fast(&roll_positions)?);
    Ok(())
}


fn step1(roll_positions: &[RollPosition] )->Result<u32,Box<dyn Error>>{
    
    Ok(calculate_accesible_rolls(roll_positions)?.len() as u32)
}
fn step2(roll_positions: &[RollPosition])->Result<u32,Box<dyn Error>>{
    let mut pending=HashSet::new();
    pending.extend(roll_positions.iter());
    let mut result=0;
    loop{
        let mut current_rolls=Vec::new();
        current_rolls.extend(pending.iter());
        let to_remove=calculate_accesible_rolls(&current_rolls)?;
        if to_remove.is_empty(){
            break;
        }else{
            result+=to_remove.len() as u32;
            for pos in to_remove{
                pending.remove(&pos);
            }
        }
    }
    Ok(result)
}

fn step2_fast(roll_positions: &[RollPosition])->Result<u32,Box<dyn Error>>{
    let mut map=HashMap::new();
    fill_adjacent_map(&mut map, roll_positions);

    let mut result=0;
    let mut to_check:HashSet<RollPosition>=map.keys().cloned().collect();
    loop{
        let mut  next_to_check:HashSet<RollPosition>=HashSet::new();

        for roll_position in to_check.iter(){
            if map.get(&roll_position).is_some_and(|&count|count<4){
                for xdiff in [-1,0,1]{
                    for ydiff in [-1,0,1]{
                        if !(xdiff==0 && ydiff==0){
                            let adj_pos=RollPosition{x:roll_position.x+xdiff,y:roll_position.y+ydiff};
                            if let Some(current_adj) = map.get_mut(&adj_pos){
                                *current_adj=*current_adj-1;
                                if *current_adj <4{
                                    next_to_check.insert(adj_pos);
                                }
                            }
                        }
                    }
                }
                map.remove(roll_position);
                result+=1;
            }
        }
        if next_to_check.len()>0{
            to_check.clear();
            to_check.extend(next_to_check.iter());
        }else{
            break;
        }
    }
    Ok(result)
}

fn calculate_accesible_rolls(roll_positions: &[RollPosition] )->Result<Vec<RollPosition>,Box<dyn Error>>{
    let mut map=HashMap::new();
    fill_adjacent_map(&mut map, roll_positions);
    let mut result=Vec::new();
    for (pos,count) in map{
        if count<4 {
            result.push(pos);
        }
    }
    Ok(result)

}

fn fill_adjacent_map(map:&mut HashMap<RollPosition,u8>,roll_positions: &[RollPosition] ){
    for &pos in roll_positions{
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
#[derive(PartialEq, Eq, Hash,Clone,Copy)]
struct RollPosition{
    x:i32,y:i32
}