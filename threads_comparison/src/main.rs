use std::{sync::{Arc, Mutex}, thread, time::Duration};
use rand::Rng;

const COUNTER_LIMIT:u32 = 30; //Limite del Contador
const THREADS_NUMBER:u32 = 5; //Número de hilos a usar

fn main() {
    let mut handles = vec![];   //Declaramos un vector de hilos para su control adecuado
    let shared_counter = Arc::new(Mutex::new(0));   //Declaramos nuestro contador compartido y mutex

    for i in 1..=THREADS_NUMBER{    //Empezamos con la creación de cada uno de nuestros hilos
        let shared_counter = Arc::clone(&shared_counter); //Por cada hilo a crear, generamos un puntero inteligente para
                                                                                //Poder compartir múltiples referencias de nuestro contador a los diferentes hilos
        let handle = thread::spawn(move ||{     //Creamos nuestro hilo
            let mut rng = rand::rng();  //Inicializamos un método de tiempos aleatorios para usarse despues.
            loop {
                let mut counter = shared_counter.lock().unwrap();   //Iniciamos un loop (while(true)) de tal manera que así
                //Debido a que shared_counter tambien es nuestro mutex, cuando el        //los hilos se encuentren únicamente incrementado el contador compartido
                //primer hilo entra, bloquea el acceso a los demás, de modo que asi 
                //puede controlar este mismo hasta que llegue al final del loop.
                
                if *counter >= COUNTER_LIMIT{   //Cuando el counter compartido sea identico al límite estipulado
                    drop(counter);              //simplemente rompera el bucle loop y liberara el mutex para que los
                    break;                      //demás hilos tambien terminen.
                }

                *counter += 1;  //En caso de que la validación anterior no se cumpla, se incrementa el contador y se imprime
                println!("Hilo {i} incrementó a {counter}"); //el ID del hilo que lo incremento y el estado actual del mismo.

                let tsleep = rng.random_range(1..=3);   //Con base a nuestra variable RNG, generamos un número aleatorio de 1s a 3s
                println!("Hilo {i} dormirá por {tsleep} segundos...");  //Mostramos en pantalla el tiempo que el hilo dormira
                drop(counter);  //Liberamos el mutex para que otro hilo pueda realizar cambios al contador
                
                thread::sleep(Duration::from_secs(tsleep)); //El hilo duerme RNG segundos indicados, de modo que cuando despierte, el proceso vuelva a repetirse
            }

            println!("Hilo {i} ha finalizado.") //Cuando el loop haya finalizado, se imprime en pantalla el ID del hilo finalizado.
        });
        handles.push(handle); //Cada uno de los hilos creados es agregado a nuestro vector de hilos
    }

    for handle in handles{
        handle.join().unwrap() //Llegado a esta parte, iteramos sobre cada uno de los hilos de nuestro vector
    }                           //Esto con el fin de únicamente esperar su finalización y acabar el programa de forma segura.

    println!("END\nValor final del contador: {}",*shared_counter.lock().unwrap()); //Imprimimos en pantalla el contador para validar que no excedio el límite
}