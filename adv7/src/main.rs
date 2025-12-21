use std::{collections::{ BTreeMap, BTreeSet, HashMap, HashSet, VecDeque}, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_to_string("input.txt")?;
    let manifold=Manifold::parse(&content)?;
    println!("step1 {}",step1(&manifold));
    println!("step2 {}",step2(&manifold)?);
    println!("step2_gemini {}",step2_gemini(&manifold));
    Ok(())
}


fn step1(manifold:&Manifold)->u32{
    let mut result=0;
    let mut beams=BTreeSet::new();
    beams.insert(manifold.beam_origin);
    for line_splitters in manifold.splitters.iter(){
        let mut next_beams=BTreeSet::new();
        for beam in beams.iter(){
            if line_splitters.contains(&beam){
                next_beams.insert(beam-1);
                next_beams.insert(beam+1);
                result+=1;
            }else{
                next_beams.insert(*beam);
            }
        }
        beams.clone_from(&next_beams);
    }
    result
}

fn add_value_to_splits(pos:usize,value:DownStates, splits: &mut BTreeMap<usize,HashSet<DownStates>>){
    let mut states=HashSet::new();
    if splits.contains_key(&pos){
        states.clone_from(splits.get(&pos).unwrap());
    }
    states.insert(value);
    splits.insert(pos, states);

}

fn step2(manifold:&Manifold)->Result<u128,Box<dyn Error>>{
    
    let mut beams=BTreeSet::new();
    beams.insert(manifold.beam_origin);
    let mut splits_produced:VecDeque<BTreeMap<usize,HashSet<DownStates>>>=VecDeque::new();
    for line_splitters in manifold.splitters.iter(){
        let mut next_beams: BTreeSet<usize>=BTreeSet::new();
        let mut splits_produced_line: BTreeMap<usize,HashSet<DownStates>>=BTreeMap::new();
        for beam in beams.iter(){
            if line_splitters.contains(&beam){
                next_beams.insert(beam-1);
                add_value_to_splits(*beam-1,DownStates::SplitLess,&mut splits_produced_line);
                next_beams.insert(beam+1);
                add_value_to_splits(*beam+1,DownStates::SplitMore,&mut splits_produced_line);
            }else{
                next_beams.insert(*beam);
                add_value_to_splits(*beam,DownStates::Direct,&mut splits_produced_line);
            }
        }
        beams.clone_from(&next_beams);
        println!("splits_produced_line {:?}",splits_produced_line);
        splits_produced.push_front(splits_produced_line);
    }
    println!("beams {:?}",beams);
    let mut beams_reverse:BTreeMap<usize,u128>=BTreeMap::new();
    beams.iter().for_each(|&pos|{beams_reverse.insert(pos,1_u128);});
    for splits_produced_line in splits_produced{
        let mut beams_reverse_line:BTreeMap<usize,u128>=BTreeMap::new();
        for (beam,split_type_map) in splits_produced_line{
            for down_state in split_type_map{
                match down_state {
                    DownStates::SplitLess=>{
                        let pos=beam+1;
                        let paths=beams_reverse.get(&beam).ok_or_else(||format!("Can't find beam {}",beam))?;
                        beams_reverse_line.entry(pos)
                            .and_modify(|v| *v=*v+*beams_reverse.get(&beam).unwrap())
                            .or_insert(*paths);
                    },
                    DownStates::SplitMore=>{
                        let pos=beam-1;
                        let paths=beams_reverse.get(&beam).ok_or_else(||format!("Can't find beam {}",beam))?;
                        beams_reverse_line.entry(pos)
                            .and_modify(|v| *v=*v+*beams_reverse.get(&beam).unwrap())
                            .or_insert(*paths);
                    },
                    DownStates::Direct=>{
                        let pos=beam;
                        let paths=beams_reverse.get(&beam).ok_or_else(||format!("Can't find beam {}",beam))?;
                        beams_reverse_line.entry(pos)
                            .and_modify(|v| *v=*v+*beams_reverse.get(&beam).unwrap())
                            .or_insert(*paths);
                    }
                    
                }
            }
        }
        
        
        println!("beams_reverse:{:?}",beams_reverse);
        beams_reverse.clone_from(&beams_reverse_line);

    }

    beams_reverse.get(&manifold.beam_origin).map(|&res|res) .ok_or("Can't find result as beam origin".into())
    
}

fn step2_gemini(manifold: &Manifold) -> u128 {
    // Mapa: { Posición : Cantidad de caminos que llegan aquí }
    // Iniciamos con 1 camino en el origen
    let mut counts: HashMap<usize, u128> = HashMap::new();
    counts.insert(manifold.beam_origin, 1);

    for line_splitters in &manifold.splitters {
        let mut next_counts: HashMap<usize, u128> = HashMap::new();

        for (pos, count) in counts {
            if line_splitters.contains(&pos) {
                // El flujo se divide: sumamos la cantidad actual a las posiciones izq y der
                // .entry().or_default() inicializa en 0 si no existe y luego suma
                *next_counts.entry(pos - 1).or_default() += count;
                *next_counts.entry(pos + 1).or_default() += count;
            } else {
                // El flujo sigue recto
                *next_counts.entry(pos).or_default() += count;
            }
        }
        
        // Aquí estaba el error. Simplemente actualizamos el mapa para la siguiente iteración.
        counts = next_counts;
        println!("counts {:?}",counts);
    }

    // La respuesta es la suma total de todos los caminos que llegaron al final
    counts.values().sum()
}

struct Manifold{
    beam_origin:usize,
    splitters: Vec<BTreeSet<usize>>
}

impl Manifold {
    fn parse(content:&str)->Result<Manifold,Box<dyn Error>>{
        let mut beam_origin=None;
        let mut splitters=Vec::new();

        for line in content.lines(){
            if beam_origin.is_none(){
                beam_origin=line.find('S');
            }else{
                let mut line_splitter=BTreeSet::new();
                for char_index in line.char_indices(){
                    if let (position,'^')=char_index{
                        line_splitter.insert(position);
                    }
                }
                splitters.push(line_splitter);
            }
        }
        Ok(Manifold{
            beam_origin:beam_origin.ok_or("No beam origin found")?,splitters:splitters
        })
    }
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn test_input_step1(){
        let content = read_to_string("test.txt").unwrap();
        let manifold = Manifold::parse(&content).unwrap();
        assert_eq!(21,step1(&manifold));
    }
    #[test]
    fn test_input_step2(){
        let content = read_to_string("test.txt").unwrap();
        let manifold = Manifold::parse(&content).unwrap();
        assert_eq!(40,step2(&manifold).unwrap());
    }

    #[test]
    fn test_input_step2_gemini(){
        let content = read_to_string("test.txt").unwrap();
        let manifold = Manifold::parse(&content).unwrap();
        assert_eq!(40,step2_gemini(&manifold));
    }
}
#[derive(Clone,PartialEq, Eq,Hash,Debug)]
enum DownStates{
    SplitLess,SplitMore,Direct
}