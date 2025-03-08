#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void check_memory(int *ptr, int N){ //Esta funcion recibe nuestro espacio de memoria y imprime todos los enteros almacenados en esta
    for(int i=0; i<N; i++){
        printf("Dato %d con el valor %d\n",i,ptr[i]);   
    }
}

int main(){
    int* ptr;       //Inicializamos nuestro puntero
    int N = 10;     //Con esta variable controlamos cuantos enteros se podrán agregar a nuestro espacio de memoria

    srand(time(NULL)); //Inicializamos la semilla de random

    ptr = (int*)malloc(N*sizeof(int)); //Creamos un espacio de memoria y hacemos que el puntero tenga acceso a este
                                                    //Este espacio podrá almacenar N numero de enteros. 
    if(ptr == NULL){
        printf("Error al asignar memoria"); //Validamos que si se haya asignado correctamente
        return 1;
    }

    for(int i=0; i<N; i++){
        ptr[i] = rand() % 100;  //Asignamos a nuestro espacio de memoria N enteros random
    }

    check_memory(ptr, N);   //Imprimimos nuestros datos para corrobar si se almacenaron
    free(ptr);              //Al ver que si funciono, liberamos la memoria

    check_memory(ptr, N);   //Para observar un problema de "use-after-free",
                            //Intentamos imprimir de nuevo los numeros almacenados en esta.
                            //Esta es la parte donde Valgrind encontrará el problema 
    return 0;
}