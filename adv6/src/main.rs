use std::{char, error::Error, fs::read_to_string, u128};

fn main() -> Result<(),Box<dyn Error>>{
    let content=read_to_string("input.txt")?;
    println!("step1 {}",process_problems(&parse_content_step_1(&content)?)?);
    println!("step2 {}",process_problems(&parse_content_step_2(&content)?)?);
    Ok(())
}


fn process_problems(input:&[Problem])->Result<u128,Box<dyn Error>>{
    let mut result=0;
    for problem in input{
        let  initial_value;
        let function:fn(u128,&u128)->u128;

        match problem.sign {
            Some('*') =>{
                initial_value=1_u128;
                function=|acc,value| acc*value;
            }
            Some('+')=>{
                initial_value=0_u128;
                function=|acc,value|acc+value;
            }
            Some(_)|None=>{
                return Err(format!("Problem has no sign {:?}",problem).into());
            }
        }
        result +=problem.operands.iter().fold(initial_value, function);
        
    }


    Ok(result)
}

fn parse_content_step_1(content:&str)-> Result<Vec<Problem>,Box<dyn Error>>{
    let mut result=Vec::new();
    for line in content.lines(){
        let mut result_num=0;
        for fragment in line.split(' '){
            if fragment.ne(""){
                while result.len()<=result_num{
                    
                    result.push(Problem{
                        operands:Vec::new(),
                        sign:None
                    });
                }
                let problem=result.get_mut(result_num).unwrap();
                if let Ok(number)=fragment.parse::<u128>(){
                    problem.operands.push(number);
                }else{
                    problem.sign=fragment.chars().nth(0).clone();
                }
                result_num+=1;
            }
        }
    }

    Ok(result)
}


fn parse_content_step_2(content:&str)-> Result<Vec<Problem>,Box<dyn Error>>{
    let partial_result:Vec<Vec<char>>=content.lines().map(|line| line.chars().collect()).collect();
    //let partial_result:Vec<&str>=content.lines().collect();
    let max_length=partial_result.iter().map(|str|str.len()).max().ok_or("Can't find max length")?;
    let mut result=Vec::new();
    let mut current_problem=Problem::new();
    for col in 0..max_length {
        let mut acc=None;
        for row in 0..partial_result.len(){
            match partial_result.get(row).map(|line|line.get(max_length-col-1)){
                Some(Some(character))=>{
                    if character.is_digit(10){
                        if acc.is_none() {acc=Some(0_u128)}
                        
                        acc=acc.or_else(||Some(0_u128)) .map(|v|   v*10+character.to_digit(10).unwrap() as u128);
                    }else if ['+','*'].contains(character){
                        current_problem.sign=Some(*character);
                    }
                },
                _default=>{}
            }
        }
        if let Some(operand)=acc {
            current_problem.operands.push(operand);
        }
        if current_problem.sign.is_some(){
            result.push(current_problem);
            current_problem=Problem::new();
        }
    }    
    
    Ok(result)
}
#[derive(Debug,PartialEq, Eq)]
struct Problem{
    operands: Vec<u128>,
    sign: Option<char>
}

impl Problem {
    fn new()->Problem{
        Problem{
            operands:Vec::new(),
            sign:None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_input_step_2(){
        let content=read_to_string("test.txt").unwrap();
        assert_eq!(3263827,process_problems(&parse_content_step_2(&content).unwrap()).unwrap());
    }
    #[test]
    fn test_parse_test_input(){
        let content=read_to_string("test.txt").unwrap();
        let result=parse_content_step_2(&content).unwrap();
        assert_eq!(result.len(),4);
        assert_eq!(
            *result.get(0).unwrap(),
            Problem{
                operands:vec![4,431,623],sign:Some('+')
            });
        assert_eq!(
            *result.get(1).unwrap(),
            Problem{
                operands:vec![175,581,32],sign:Some('*')
            });
    }


}