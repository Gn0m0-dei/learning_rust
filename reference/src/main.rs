fn main() {
    let s1 = String::from("hola");
    let len = calcular_longitud(&s1);
    println!("La longitud de '{s1}' es {len}.");

    let mut s = String::from("hola");
    modificar(&mut s);
    modificar2(&mut s);
    println!("{s}.");

    let mut s = String::from("hola");
    {
        let r1 = &mut s;
    } // r1 se sale de su ámbito aquí, por lo que no hay problema
    // si creamos otra referencia mutable
    let r2 = &mut s;

    let mut s = String::from("hello");
    let r1 = &s; // no hay problema
    let r2 = &s; // no hay problema
    println!("{r1} y {r2}");
    // variables r1 y r2 no se usaran más a partir de aquí
    let r3 = &mut s; // no hay problema
    println!("{r3}");

    no_colgante();
}

fn calcular_longitud(s: &String) -> usize {
    // es una referencia a un String
    s.len()
} // Aquí, s sale de ámbito. Pero como no tiene el ownership/la propiedad sino
// que s es solo un prestamo, no se destruye, se regresa al propietario, s1.

fn modificar(un_string: &mut String) {
    un_string.push_str(", mundo");
}

fn modificar2(un_string: &mut String) {
    un_string.push_str(", mundo");
}

fn no_colgante() -> String {
    let s = String::from("hola");

    s // &s error: could not compile `reference` (bin "reference") due to 1 previous error; 2 warnings emitted
}
