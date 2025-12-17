use std::{error::Error, fs::read_to_string, u128};

fn main() -> Result<(),Box<dyn Error>>{
    let content=read_to_string("input.txt")?;
    let input=parse_content(&content)?;
    println!("step1 {}",step1(&input)?);
    Ok(())
}


fn step1(input:&[Problem])->Result<u128,Box<dyn Error>>{
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

fn parse_content(content:&str)-> Result<Vec<Problem>,Box<dyn Error>>{
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
#[derive(Debug)]
struct Problem{
    operands: Vec<u128>,
    sign: Option<char>
}