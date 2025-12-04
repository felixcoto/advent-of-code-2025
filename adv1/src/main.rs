use std::{fs::File, io::{BufRead, BufReader}, num::ParseIntError};

fn main() {
    match step1(){
        Ok(result)=>println!("Step 1:{}",result),
        Err(error_message)=>println!("Step 1:{}",error_message)
    }

    match step2("input.txt"){
        Ok(result)=>println!("Step 2:{}",result),
        Err(error_message)=>println!("Step 2:{}",error_message)
    }
}

fn step1()->Result<u32,String>{
    let buf_reader =  BufReader::new(File::open("input.txt").expect("can't find file"));
    let mut position:i32=50;
    let mut result:u32=0;
    for line in buf_reader.lines(){
        let line= line.map_err(|e| format!("Error reading line {}",e))?;
        

        let step=parse_step(&line)
            .map_err(|error_message| format!("Error parsing line {}. {}",line,error_message))?;

        match step.direction{
            Direction::L=>position=position-step.quantity as i32,
            Direction::R=>position=position+step.quantity as i32
        }
        
        position=position.rem_euclid(100);
        if position==0 {
            result+=1;
        }
    }
    Ok(result)
}

fn step2(file_name:&str)->Result<u32,String>{
    let buf_reader =  BufReader::new(File::open(file_name).expect("can't find file"));
    let mut position:i32=50;
    let mut result:u32=0;
    for line_result in buf_reader.lines(){
        let line;
        match line_result {

            Ok(_line)=>line=_line,Err(read_error)=>return Err(format!("Error reading line {}",read_error))
        }

        match parse_step(&line){
            Ok(step)=>{
                let complete_rotations=step.quantity/100;
                result+=complete_rotations;
                let remainder=step.quantity.rem_euclid(100).cast_signed();
                let last_position=position;
                match step.direction{
                    Direction::L=>{
                        position=position-remainder;
                        if position<=0 && last_position!=0{
                            result+=1;
                        }
                    },
                    Direction::R=>{
                        position=position+remainder;
                        if position>=100 && last_position!=0{
                            result+=1;
                        }
                    }
                }
            },
            Err(error_message)=>{
                return Err(format!("Error parsing line {}. {}",line,error_message));
            }
        }
        position=position.rem_euclid(100);
        //println!("{}:position {} result {}",line,position,result);
        
    }
    Ok(result)
}

fn parse_step(line: &String) -> Result<PassStep,String>{
    if  line.len()>=2 {
        let (direction_str,quantity_str)= line.split_at(1);
        let mut result= PassStep{
            direction:Direction::L,quantity:0
        };
        let quantity:Result<u32,ParseIntError>=quantity_str.parse();
        match quantity{
            Err(_)=>return Err(format!("The value {} is not a number",quantity_str)),
            Ok (quantity_number)=>result.quantity=quantity_number
        }
        if direction_str.eq("R"){
            result.direction=Direction::R
        }else if !direction_str.eq("L"){
            return Err("La dirección ".to_owned()+direction_str+" no es correcta");
        }
        Ok(result)
    }else{
        return Err("step has to be more than 1 character".to_string());
    }
}

struct PassStep{
    direction:Direction,
    quantity: u32
}

enum Direction{
    L,R
}