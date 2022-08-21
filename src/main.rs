use regex::Regex;

fn main() {
    
    // Regex
    let re_op:Regex= Regex::new(r"(\d+)\s*([\*\+\-])\s*(\d+)").unwrap();

    // Datos del usuario
    println!("Introduce tu operación: ");
    let mut operacion:String = String::new(); 
    std::io::stdin().read_line(&mut operacion).unwrap();

    // Operaciones matemáticas

    loop {
        let caps = re_op.captures(operacion.as_str());
        
        // Si no tenemos más operaciones, terminamos
        if caps.is_none(){
            break;
        }
        // Hacemos operaciones paso a paso
        let caps = caps.unwrap();
        let left : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let op : &str = caps.get(2).unwrap().as_str();
        let right : i32 = caps.get(3).unwrap().as_str().parse().unwrap();

        let aux_op : i32 = if op == "*" {
            left * right
        }else if op == "/"{
            left / right
        }else if op == "+"{
            left + right
        }else {
            left - right
        };

        operacion = operacion.replace(caps.get(0).unwrap().as_str(), &aux_op.to_string())
    }


    //Mostramos resultados
    println!("Resultado: {}",operacion);
}