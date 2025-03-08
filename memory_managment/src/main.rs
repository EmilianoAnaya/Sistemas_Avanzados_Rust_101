use rand::Rng;

fn check_memory(memory: &[i32]){ //Esta funcion recibe nuestro espacio de memoria y imprime todos los enteros almacenados en esta
    for (i, &data) in memory.iter().enumerate(){
        println!("Dato {i} con el valor {data}");
    }
}

fn main() {
    const N: usize = 10;    //Con esta variable controlamos el numero de enteros disponibles en nuestro bloque de memoria
    let mut memory = Vec::with_capacity(N);   //En Rust, no es posible implementar punteros de forma explicita
                                                        //Por ello, nuestra memoria únicamente será un vector con un tamaño fijo
    let mut rng = rand::rng(); //Inicializamos un generador de numeros aleatorios 

    for _ in 0..N{
        memory.push(rng.random_range(1..100));  //Asignamos a nuestro espacio de memoria N enteros random
    }

    check_memory(&memory); //Imprimimos nuestros datos para corrobar si se almacenaron

    drop(memory);   //Al ver que si funciono, liberamos la memoria

    // check_memory(&memory);   //No nos deja compilar debido a que ya se libero
                                //la memoria anteriormente, evitandose así la demonstración de un
                                //problema "user-after-free".
}
