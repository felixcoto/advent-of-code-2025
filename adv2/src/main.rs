use std::{fs::File, io::Read, u128};

fn main() {
    match step1_file("input.txt"){
        Ok(result)=>println!("step 1:{}",result),
        Err(message)=>println!("step 1 error:{}",message)

    }
}

fn step1_file(file_name: &str)-> Result<u128,String> {
    let mut file=File::open(file_name).map_err(|e| format!("Can't open file {},err {}",file_name,e))?;
    let mut buf=String::new();
    file.read_to_string(&mut buf).map_err(|e|format!("Error reading file {}, err {}",file_name,e))?;
    Ok(step1_str(&buf)?)
}

fn step1_str(value:&str)-> Result<u128,String>{
    let mut buf=value.to_owned();
    let mut result:u128=0;
    while buf.len()>0 {
        let new_range;
        match buf.find(","){
            Some(position)=>{
                new_range=buf.drain(0..position).collect();
                buf.drain(0..1);
            },
            None=>{
                new_range=buf.to_string();
                buf.drain(0..);
            }
        }
        let dash_position=new_range.find("-").ok_or_else(||format!("There is no dash in range {}",new_range))?;
        let range_init_str=&new_range[0..dash_position];
        let range_end_str=&new_range[dash_position+1..];
        let range_init:u128=range_init_str.parse().map_err(|e| format!("can't parse {} as number, {}",range_init_str,e))?;
        let range_end:u128=range_end_str.parse().map_err(|e| format!("can't parse {} as number, {}",range_end_str,e))?;
        for current_number in range_init..=range_end{
            let current_number_str=current_number.to_string();
            if current_number_str.len()%2==0{
                let current_number_start=current_number_str[0..current_number_str.len()/2].to_string();
                let current_number_end=current_number_str[(current_number_str.len()/2)..].to_string();
                if current_number_start==current_number_end{
                    result+=current_number;
                }
            }
        }
    }
    Ok(result)
    
}


#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn test_11_to_22(){
        let result=step1_str("11-22").unwrap();
        assert_eq!(result,33);
    }
}