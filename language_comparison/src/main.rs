use std::io;

fn decimal_binary(mut decimal:i32){
    let mut binary: Vec<i32> = Vec::new();  //Generamos un arreglo dinámico (Vector) para guardar los bits de conversion

    println!("Decimal: {decimal}"); //Imprimimos nuestro decimal como mensaje de apoyo

    if decimal == 0{
        println!("Binary: 0\n\n\n");    //Si nuestro decimal es igual a 0, imprimimos 0 y retornamos
        return;
    }

    while decimal > 0 {
        binary.push(decimal%2); //Si nuestro decimal es diferente de 0, añadimos los bits correspondientes a nuestro vector
        decimal = decimal/2;    //Y cambiamos el resultado de nuestro decimal sobre 2.
    }

    print!("Binary: ");
    for bit in binary.iter().rev(){ 
        print!("{bit}");    //Una vez finalizado, recorremos nuestro vector en reversa para mostrar el orden de los bits
    }                       //de forma correcta

    println!("\n\n")  //Retornamos para continuar el funcionamiento del código

}

fn main() {
    let mut input:String = String::new(); //Declaramos nuestro input para ingresar el numero entero para la conversion

    loop {
        println!("Decimal to Binary\n|Enter| Enter Number   |Ctrl+C| Finish\nInsert your Decimal: "); //Hacemos un menu donde se le indica al usuario los controles del menu.
        io::stdin()
            .read_line(&mut input)  //Generamos un stdin para leer el texto a ingresar.
            .expect("Failed to Read");

        let decimal: i32 = input.trim().parse().expect("Invalid Input");    //Una vez generado el input, quitamos los espacios inncesarios
                                                                                //Y lo parseamos a un número entero.
        decimal_binary(decimal);    //Llamamos a nuestra función de decimal a binario

        input.clear();  //Limpiamos el input para que se pueda volver a escribir sobre el.
    }
}
