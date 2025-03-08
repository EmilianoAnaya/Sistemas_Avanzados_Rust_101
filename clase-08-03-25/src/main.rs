fn main() {

    //let mut x = 5;
    //x = 10;

    //let mut y = 10;
    // y = 20;

    let x = 5;
    {
        let x = x+1;
        println!("(in scope) El valor de x es: {}", x);
    }
    println!("(out of scope) El valor de x es: {}", x);

    let entero: i32 = 42;
    let flotante: f32 = 3.14;
    let boleano: bool = true;
    let caracter: char = 'a';
    let tupla: (i32, f32, char) = (78, 3.1416, 'b');
    let arreglo: [i32; 3] = [1, 2, 3];

    println!("Hello, world!");
    println!("entero: {entero}\nflotante: {flotante}\nboleano: {boleano}\ncaracter {caracter}\ntupla: {:?}\narreglo: {:?}", tupla, arreglo);

    println!("Imprimiendo un tupla forma 1: {:?}", tupla); //Esta notacion escribe la tupla de forma cruda, i.e 'b' -> 'b'
    println!("Imprimiendo un tupla forma 2: ({}, {}, {})", tupla.0, tupla.1, tupla.2); //Esta notación escribe de forma correcta i.e 'b' -> b
}
