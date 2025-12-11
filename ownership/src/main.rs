fn main() {
    // STACK
    let mut x = "hello";
    x = "hello2"; // reference change
    print!("valor de x: {x}");

    // HEAP
    let mut y = String::from("world");
    y.push_str("!"); // real mutation (string hint is the name of the push_str param)
    print!("valor de y: {y}");

    // Move vars (stack)
    let x = 5;
    let y = x; // copy, 2 vars same value (stack)
    println!("{x}, {y}!"); // works

     // Move vars (heap)
    let s1 = String::from("hola");
    let s2 = s1; // move s1 to s2
    // println!("{s1}, mundo!");  borrow of moved value: s1 value borrowed here after move (rustc E0382)
    println!("{s2}, mundo!"); // works
    /*
    s1: (removed when s1 asign to s2 but not the memory)
    +--------+-------+--------+----------+----------+
    | nombre | valor | puntero| longitud | capacidad|
    +--------+-------+--------+----------+----------+
    |   s1   |       | 0x1234 |    4     |    4     |
    +--------+-------+--------+----------+----------+

    s2:
    +--------+-------+--------+----------+----------+
    | nombre | valor | puntero| longitud | capacidad|
    +--------+-------+--------+----------+----------+
    |   s2   |       | 0x1234 |    4     |    4     |
    +--------+-------+--------+----------+----------+

    Heap: (memory) 0x1234
    +-------+-------+
    |indice | valor |
    +-------+-------+
    |   0   |   h   |
    |   1   |   o   |
    |   2   |   l   |
    |   3   |   a   |
    +-------+-------+
    */

    // drop s and move new string to s
    let mut s = String::from("hola");
    s = String::from("ahoy");
    println!("{s}, mundo!");

    //  s1 -> memory1 copy to 2 new s2 -> memory2
    let s1 = String::from("hola");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    //FUNCTIONS
    let s = String::from("hola");       // s aparece en el ámbito
    tomar_ownership(s);              // El valor de s se mueve a la función...
                                                // ... y ya no es valido aquí
    let x = 5;                             // x aparece en el ámbito
    hacer_una_copia(x);              // x deberia moverse a la función,
                                                // pero i32 implementa Copy, entonces es
    println!("{x}");                            // valido aún despues de llamar a la función

    let s1 = da_un_ownership2();         // da_un_ownership es llamado y
                                        // devuelve el valor de retorno
                                        // a s1
    let s2 = String::from("hola");     // s2 aparece en el ámbito
    let s3 = toma_y_devuelve2(s2);  // s2 es movido a la función
                                        // toma_y_devuelve, que también
                                        // retorna el valor de s2 a s3

}// Aquí termina el ámbito, x es destruido con drop. La memoria es liberada.
  // s ya no existia porque habia sido movido a la función.
  // Nada especial ocurre.
  //
  // Fin el ámbito, s3 es destruido con drop y se libera la memoria.
  // s2 fue movido previamente, entonces no pasa nada.
  // s1 es destruido con drop y se libera la memoria.

fn tomar_ownership(un_string: String) { // un_string aparece en el ámbito
    println!("{un_string}");
} // Aquí termina el ámbito, un_string es destruido con drop.
  // La memoria es liberada.

fn hacer_una_copia(un_entero: i32) { // un_entero aparece en el ámbito
    println!("{un_entero}");
} // Aquí termina el ámbito, un_entero es destruido. Nada especial ocurre.

fn da_un_ownership2() -> String {             // da_un_ownership mueve su
                                             // retorno a la función que la
                                             // llama
    let un_string = String::from("tuyo");    // un_string aparece en el ámbito

    un_string                                // un_string es retornado y
                                             // mueve su valor
}

// Esta función toma un String y devuelve uno
fn toma_y_devuelve2(un_string: String) -> String { // un_string aparece
                                                  // en el ámbito

    un_string  // un_string es retornado y mueve su valor
}
