use std::{error::Error, fs};

fn main() {
    let complete=leer_archivo_completo("input.txt").expect("Can't open file");
    let batteries_packs=parse_complete_string(&complete).unwrap();
    println!("step 1:{}",step1(&batteries_packs));
    println!("step 2:{}",step2(&batteries_packs));

}

fn parse_complete_string(complete:&str)->Result<Vec<Vec<u32>>,Box< dyn Error>>{
    let mut result=Vec::new();
    for line in complete.lines(){
        result.push(parse_line(line)?);
    }
    Ok(result)
}

fn parse_line(line :&str)->Result<Vec<u32>,Box<dyn Error>>{
    let mut result=Vec::new();
    for character in line.chars(){
        result.push(character.to_digit(10).ok_or_else(||format!("character {} is not a number",character))?);
    }
    Ok(result)
}


fn step1(batteries_packs: &[Vec<u32>] )->u128{
    let mut result:u128=0;
    for battery_pack in batteries_packs.iter(){
        let mut max_pack_result=0;
        for first_position in 0..battery_pack.len()-1{
            for second_position in first_position+1..battery_pack.len(){
                let current_pack_result=battery_pack[first_position]*10+battery_pack[second_position];
                if current_pack_result>max_pack_result {
                    max_pack_result=current_pack_result;
                }
            }
        }
        result=result+max_pack_result as u128;
    }
    return result;
}

fn step2(batteries_packs: &[Vec<u32>] )->u128{
    let mut result:u128=0;

    for battery_pack in batteries_packs.iter(){
        let current=step2_one_pack(battery_pack, 12);
        if current.is_some(){
            result+=current.unwrap();
        }else{
            println!("Can't find value for battery pack {:?}",battery_pack);
        }
    }
    result
}


fn step2_one_pack(battery_pack:&[u32],batteries_to_activate:usize)->Option<u128>{
    let mut finded_digits=None;
    let mut digit=9;
    while finded_digits==None && digit>0{
        let current_find=step2_find_digit_and_position(battery_pack, digit, batteries_to_activate);
        if current_find.is_empty(){
            digit-=1;
        }else{
            finded_digits=Some(current_find);
        }
    }

    if finded_digits.is_some(){
       
        if batteries_to_activate==1{
            return Some(digit as u128);
        }else{
            let result=finded_digits
                .unwrap()
                .iter()
                .map(|pos|step2_one_pack(&battery_pack[pos+1..], batteries_to_activate-1))
                .filter(|o|o.is_some())
                .map(|o|o.unwrap())
                .max()
                .map(|res|(digit as u128)*(10_u128.pow(batteries_to_activate as u32-1))+res)?;


            //println!("test {}",result);
            return Some(result);
        }



        
    }else{
        return None;
    }
    
}

fn step2_find_digit_and_position(battery_pack:&[u32],digit:u32,min_digits_remaining:usize)->Vec<usize>{
    let mut result=Vec::new();
    for position in 0..=battery_pack.len()-min_digits_remaining{
        if battery_pack[position]==digit {
            result.push(position);
        }
    }
    return result;
}
fn leer_archivo_completo(ruta: &str) -> Result<String, Box<dyn Error>> {
    // fs::read_to_string abre el archivo, lee todo su contenido
    // y lo devuelve como un Result<String, io::Error>.
    let contenido = fs::read_to_string(ruta)?; 
    
    Ok(contenido)
}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn step2_test_987654321111111(){
        let battery_pack=parse_line("987654321111111").unwrap();
        let result=step2_one_pack(&battery_pack, 12).unwrap();
        assert_eq!(result,987654321111_u128);
    }

     #[test]
    fn step2_test_818181911112111(){
        let battery_pack=parse_line("818181911112111").unwrap();
        let result=step2_one_pack(&battery_pack, 12).unwrap();
        assert_eq!(result,888911112111_u128);
    }
    
}