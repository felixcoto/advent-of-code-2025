use std::{error::Error, fs};

use crate::point2d::Point2D;

mod point2d;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input.txt")?;
    let points = parse_content(&content)?;
    println!("step1 {:?}",step1(&points));
    println!("step2 {:?}",step2(&points));
    println!("step2_gemini {:?}",step2_gemini(&points));
    Ok(())
}

fn parse_content(content:&str)->Result<Vec<Point2D>,Box<dyn Error>>{
    content.lines().map(|line|line.parse::<Point2D>().map_err(|e|e.into())).collect()
}

fn step1(points:&[Point2D])->Option<u128>{
    let mut result=None;
    if points.len()>=2{
        for first_index in 0..points.len()-1{
            for second_index in first_index+1..points.len(){
                let current_area=points[first_index].area_other_tiles(&points[second_index]);
                result=Some(result.map_or(current_area, |v: u128|v.max(current_area)));
            }
        }
    }
    
    result
}


fn step2(points:&[Point2D])->Option<u128>{
    let mut result=None;
    if points.len()>=2{
        for first_index in 0..points.len()-1{
            for second_index in first_index+1..points.len(){
                let first_point=points[first_index];
                let second_point=points[second_index];
                let current_area=first_point.area_other_tiles(&second_point);
                if result.is_none() || result.unwrap()<current_area {
                    let mut debug = false;
                    if current_area==1613305596{
                        println!("Found potential result at {:?} and {:?}",first_point,second_point);
                        debug=true;
                    }
                    let lines_to_check=[
                        first_point.clone(),
                        Point2D { x: first_point.x, y: second_point.y },
                        second_point.clone(),
                        Point2D { x: second_point.x, y: first_point.y }
                    ];
                    let mut intesect = false;
                    for third_index in 0..lines_to_check.len() {
                        let current_line=Line2D{
                            origin:lines_to_check[third_index],
                            dest:lines_to_check[(third_index+1)%lines_to_check.len()]
                        };
                        
                        intesect=polygon_intersects(points, &current_line,debug);
                        if intesect {break;}
                    }

                    
                    if !intesect{
                        
                        result=Some(current_area);
                        //println!("Found potential rectangle with area {} between {:?} and {:?}, line checked {:?}", current_area, first_point, second_point,opposite_line);
                    } else {
                        //println!("Skipping intersection check for points {:?} and {:?}, line checked {:?}", first_point, second_point,opposite_line);
                    }
                }
            }
        }
    }
    result
}
#[derive(Debug)]
pub struct Line2D{
    origin:Point2D,dest:Point2D
}

impl Line2D{
    pub fn check_only_one_intersect(&self, other: &Line2D) -> bool {
        self.check_only_one_intersect_d(other, false)
    }

    pub fn check_only_one_intersect_d(&self, other: &Line2D,debug:bool) -> bool {
        let o1 = get_orientation(&self.origin, &self.dest, &other.origin);
        let o2 = get_orientation(&self.origin, &self.dest, &other.dest);
        let o3 = get_orientation(&other.origin, &other.dest, &self.origin);
        let o4 = get_orientation(&other.origin, &other.dest, &self.dest);

        // 1. Condición de intersección estándar (incluye cuando se tocan)
        let intersectan = (o1 != o2) && (o3 != o4);

        // 2. Condición de exclusión: ¿El contacto ocurre en un extremo de MI línea?
        // Si o3 es 0, mi origin está sobre la recta de la otra línea.
        // Si o4 es 0, mi dest está sobre la recta de la otra línea.
        let es_mi_extremo = (o3 == 0) || (o4 == 0);

        if debug {
            println!("o1: {}, o2: {}, o3: {}, o4: {}", o1, o2, o3, o4);
            println!("intersectan: {}", intersectan);
            println!("es_mi_extremo: {}", es_mi_extremo);
        }

        // Queremos que intersecten, pero que NO sea en mi extremo
        intersectan && !es_mi_extremo

    }

}

fn get_orientation(p:&Point2D,q:&Point2D, r:&Point2D)->u8{
    // Convert to signed integers for calculation to handle negative results correctly
    let val = (q.y as i64 - p.y as i64) * (r.x as i64 - q.x as i64)
            - (q.x as i64 - p.x as i64) * (r.y as i64 - q.y as i64);

    if val==0 {return 0;}// Colineales
    else if val > 0 {1} else {2} // 1: Clockwise, 2: Counterclockwise
}
fn polygon_intersects(points:&[Point2D],line:&Line2D,debug:bool) -> bool {
    let mut result=false;
    for first_index in 0..points.len(){
        let pol_line=Line2D{
            origin:points[first_index].clone(),
            dest:points[if first_index==(points.len()-1) {0} else{first_index+1}].clone()
        };
        let mut debug_line=false;
        if debug && first_index==217{
            debug_line=true;

        }
        result=line.check_only_one_intersect_d(&pol_line,debug_line);
        
        if result {
            if debug{
                println!("Intersection found index {:?} line {:?}",first_index, pol_line);
            }
            break;
        }
    }
    result
        
         
}


// Reemplaza tu función step2 con esta
fn step2_gemini(points: &[Point2D]) -> Option<u128> {
    let mut result = None;
    if points.len() >= 2 {
        for first_index in 0..points.len() - 1 {
            for second_index in first_index + 1..points.len() {
                let p1 = &points[first_index];
                let p2 = &points[second_index];

                // Definir límites del rectángulo
                let min_x = p1.x.min(p2.x);
                let max_x = p1.x.max(p2.x);
                let min_y = p1.y.min(p2.y);
                let max_y = p1.y.max(p2.y);

                let current_area = p1.area_other_tiles(p2);

                // Optimización rápida
                if let Some(best) = result {
                    if current_area <= best { continue; }
                }

                // 1. CHECK VÉRTICES: ¿Hay algún vértice "flotando" dentro del rectángulo?
                let mut valid = true;
                for p in points {
                    if p.x > min_x && p.x < max_x && p.y > min_y && p.y < max_y {
                        valid = false;
                        break;
                    }
                }
                if !valid { continue; }

                // 2. CHECK ARISTAS (NUEVO): ¿Alguna pared cruza el rectángulo?
                // Iteramos sobre todas las paredes del polígono
                for i in 0..points.len() {
                    let pa = &points[i];
                    let pb = &points[(i + 1) % points.len()]; // Siguiente punto (wrap)

                    // ¿Es una pared Vertical?
                    if pa.x == pb.x {
                        let edge_x = pa.x;
                        let edge_min_y = pa.y.min(pb.y);
                        let edge_max_y = pa.y.max(pb.y);

                        // Si la pared vertical está estrictamente entre la izquierda y derecha del rect
                        if edge_x > min_x && edge_x < max_x {
                            // Y si se solapa verticalmente con el rectángulo
                            let overlap_min = edge_min_y.max(min_y);
                            let overlap_max = edge_max_y.min(max_y);
                            if overlap_min < overlap_max {
                                valid = false; // La pared corta el rectángulo
                                break;
                            }
                        }
                    } 
                    // ¿Es una pared Horizontal?
                    else if pa.y == pb.y {
                        let edge_y = pa.y;
                        let edge_min_x = pa.x.min(pb.x);
                        let edge_max_x = pa.x.max(pb.x);

                        // Si la pared horizontal está estrictamente entre arriba y abajo del rect
                        if edge_y > min_y && edge_y < max_y {
                            // Y si se solapa horizontalmente con el rectángulo
                            let overlap_min = edge_min_x.max(min_x);
                            let overlap_max = edge_max_x.min(max_x);
                            if overlap_min < overlap_max {
                                valid = false; // La pared corta el rectángulo
                                break;
                            }
                        }
                    }
                }
                if !valid { continue; }

                // 3. CHECK CENTRO: ¿Estamos dentro del polígono? (Evita casos tipo "U")
                // Usamos el punto medio exacto
                let center_x = (min_x as f64 + max_x as f64) / 2.0;
                let center_y = (min_y as f64 + max_y as f64) / 2.0;

                if is_point_in_polygon(center_x, center_y, points) {
                    result = Some(result.map_or(current_area, |v: u128| v.max(current_area)));
                }
            }
        }
    }
    result
}

// Algoritmo Ray Casting para ver si un punto está dentro de un polígono
// Trazamos un rayo horizontal hacia la derecha desde (x, y) y contamos intersecciones con bordes verticales.
fn is_point_in_polygon(x: f64, y: f64, poly: &[Point2D]) -> bool {
    let mut inside = false;
    let n = poly.len();
    
    for i in 0..n {
        let p1 = &poly[i];
        let p2 = &poly[(i + 1) % n]; // El siguiente punto (con wrap-around)

        // Solo nos importan las aristas verticales para un rayo horizontal
        // Una arista es vertical si p1.x == p2.x
        if p1.x == p2.x {
            let edge_x = p1.x as f64;
            let y_min = p1.y.min(p2.y) as f64;
            let y_max = p1.y.max(p2.y) as f64;

            // Condición de intersección:
            // 1. El borde vertical está a la derecha del punto (edge_x > x)
            // 2. La 'y' del punto está comprendida en el rango vertical del borde
            if edge_x > x && y > y_min && y < y_max {
                inside = !inside;
            }
        }
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_step1() -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string("test.txt")?;
        let points = parse_content(&content)?;
        let result=step1(&points);
        assert_eq!(Some(50),result);
        Ok(())
    }
    #[test]
    fn test_step2() -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string("test.txt")?;
        let points = parse_content(&content)?;
        let result=step2(&points);
        assert_eq!(Some(24),result);
        Ok(())
    }


    #[test]
    fn test_step2_gemini() -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string("test.txt")?;
        let points = parse_content(&content)?;
        let result=step2_gemini(&points);
        assert_eq!(Some(24),result);
        Ok(())
    }
    #[test]
    fn test_check_only_one_intersect()-> Result<(), Box<dyn Error>> {
        let line=Line2D{
                    origin:Point2D { x: 9, y: 3 },
                    dest: Point2D { x: 2, y: 5 }
                };
        let line2=Line2D{
            origin:Point2D { x: 2, y: 3 },
            dest: Point2D { x: 2, y: 5 }
        };
        let result=line.check_only_one_intersect(&line2);
        assert_eq!(false,result);
        Ok(())
    }
    #[test]
    fn test_check_only_one_intersect2()-> Result<(), Box<dyn Error>> {
        let line=Line2D{
                    origin:Point2D { x: 9, y: 3 },
                    dest: Point2D { x: 2, y: 4 }
                };
        let line2=Line2D{
            origin:Point2D { x: 2, y: 3 },
            dest: Point2D { x: 2, y: 5 }
        };
        let result=line.check_only_one_intersect(&line2);
        assert_eq!(false,result);
        Ok(())
    }

    #[test]
    fn test_check_only_one_intersect3()-> Result<(), Box<dyn Error>> {
        let line=Line2D { origin: Point2D { x: 11, y: 3 }, dest: Point2D { x: 7, y: 7 } };
        let line2=Line2D{
            origin:Point2D { x: 2, y: 3 },
            dest: Point2D { x: 2, y: 5 }
        };
        let result=line.check_only_one_intersect(&line2);
        assert_eq!(false,result);
        Ok(())
    }
    #[test]
    fn test_intersect()-> Result<(), Box<dyn Error>>{
        let content = fs::read_to_string("test.txt")?;
        let points = parse_content(&content)?;
        let line=Line2D { origin: Point2D { x: 11, y: 3 }, dest: Point2D { x: 7, y: 7 } };
        let result=polygon_intersects(&points, &line, true);
        assert_eq!(true,result);
        Ok(())
    }

}
