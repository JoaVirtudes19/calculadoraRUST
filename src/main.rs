use regex::Regex;

fn realiza_parentesis(mut operacion:String) -> String {
    loop {
        let re_op:Regex= Regex::new(r"\((.+)\)").unwrap();
        let caps = re_op.captures(operacion.as_str());
        if caps.is_none(){
            break;
        }
        let caps = caps.unwrap(); //Hacemos unwrap
        let mut sub_operacion = caps.get(1).unwrap().as_str().to_string();
        // Parentesis
        sub_operacion = realiza_parentesis(sub_operacion);
        // Multiplicaciones
        sub_operacion = realiza_operacion("*", sub_operacion);
        // Divisiones
        sub_operacion = realiza_operacion("/", sub_operacion);
        // Sumas
        sub_operacion = realiza_operacion("+", sub_operacion);
        // Restas
        sub_operacion = realiza_operacion("-", sub_operacion);

        //Una vez realizadas podemos operar normal
        operacion = operacion.replace(caps.get(0).unwrap().as_str(), &sub_operacion.to_string());
    }
    return operacion;
}


fn realiza_operacion(op:&str,mut operacion:String) -> String{
    let string_regex:String = if op == "/" {
        r"(\d+)\s*/\s*(\d+)".to_string()
    } else{
        r"(\d+)\s*\".to_string()+op+&r"\s*(\d+)".to_string()
    };

    let re_op:Regex= Regex::new(&string_regex).unwrap();
    loop {
        let caps = re_op.captures(operacion.as_str());
        
        // Si no tenemos más operaciones, terminamos
        if caps.is_none(){
            break;
        }
        // Hacemos operaciones paso a paso
        let caps = caps.unwrap();
        let left : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right : i32 = caps.get(2).unwrap().as_str().parse().unwrap();

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
    return operacion
}
fn main() {
    
    // Datos del usuario
    println!("Introduce tu operación: ");
    let mut operacion:String = String::new(); 
    std::io::stdin().read_line(&mut operacion).unwrap();

    // Operaciones matemáticas

    // Parentesis
    operacion = realiza_parentesis(operacion);
    // Multiplicaciones
    operacion = realiza_operacion("*", operacion);
    // Divisiones
    operacion = realiza_operacion("/", operacion);
    // Sumas
    operacion = realiza_operacion("+", operacion);
    // Restas
    operacion = realiza_operacion("-", operacion);

    //Mostramos resultados
    println!("Resultado: {}",operacion);
}