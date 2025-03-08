#include <stdio.h>
#include <pthread.h>
#include <time.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdbool.h>

int GLOBAL_COUNTER = 0; // Contador global
int COUNTER_LIMIT = 30; // Limite del Contador
int THREADS_NUMBER = 5; // Número de Hilos a usar

pthread_mutex_t mutex; // Mutex de los hilos

void* increment(void* args){
    int id = *(int*)args;   //Recibimos el ID correspondiente del hilo
    while(true){            //Iniciamos un while true para que se encargue de incrementar el GLOBAL_COUNTER

        pthread_mutex_lock(&mutex); //Al inicio de cada ciclo, bloqueamos el mutex, de modo que haga esperar a los demás hilos siguientes al primero que entro

        if(COUNTER_LIMIT <= GLOBAL_COUNTER){    //Esta validación permite que, si se ha llegado al limite del contador estipulado,
            pthread_mutex_unlock(&mutex);       //Los hilos que entraron de nuevo al while no incrementen más allá del límite, liberando el mutex
            break;                              //Para que otro hilo proceda con la misma validación y liberación.
        }
        GLOBAL_COUNTER++;       //Se incrementa el contador global
        printf("Hilo %d incremento a: %d\n", id, GLOBAL_COUNTER);   //Se imprime el ID del hilo y el estado actual del contador.
        pthread_mutex_unlock(&mutex);   //Se libera el mutex para que otro hilo pueda incrementar el contador
        
        int tsleep = (rand() % 3)+1;                                //Para agregar un factor de aleatoriedad, se agregó la función sleep para cada hilo
        printf("El Hilo %d duerme %d segundos...\n",id, tsleep);    //De modo que ciertos hilos pueden dormir más que otros al momento
        sleep(tsleep);                                              //De hacer los incrementos en el contador
    }
    return NULL;
}


int main(){
    pthread_t threads[THREADS_NUMBER];  //Declaramos N numeros de Hilos
    int threads_ids[THREADS_NUMBER];    //Hacemos una lista aparte como una enumeración de cada hilo

    srand(time(NULL)); //Inicializamos una semilla random

    pthread_mutex_init(&mutex, NULL);   //Iniciamos el mutex del Hilo
    
    for(int i=0; i<THREADS_NUMBER; i++){
        threads_ids[i] = i;                 
        pthread_create(&threads[i],NULL, increment, &threads_ids[i]);
        //Iteremos desde 0 hasta N numero de hilos que creamos, 
        //asignandoles la funcion en común para que inicien con el incremento
    }

    for(int i=0; i<THREADS_NUMBER; i++){
        pthread_join(threads[i], NULL);
        //Esperamos cada uno de nuestros hilos mediante un los joins.
    }

    pthread_mutex_destroy(&mutex); //Destruimos el mutex una vez retornados todos los hilos.


    printf("END\nValor Final del contador: %d\n", GLOBAL_COUNTER); //Imprimos el valor total, validando que este concuerde con el límite indicado
    return 0;
}