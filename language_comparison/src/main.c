#include <stdio.h>
#include <stdbool.h>

void decimal_binary(int decimal){
    int binary[64];
    int index = 0;  //Inicializamos un arreglo de 64bits y el index para movernos a traves de este.

    printf("Decimal: %d\n", decimal);
    printf("Binary: "); //Imprimos en texto la información de ayuda.
    
    if(decimal == 0){
        printf("0\n\n\n");
        return; //Si el decimal es 0, solo imprimimos 0 y retornamos
    }

    while(decimal > 0){
        binary[index] = decimal%2;
        decimal = decimal/2;    //Si nuestro decimal es diferente de 0, añadimos los bits correspondientes a nuestro arreglo
        index++;                //Y cambiamos el resultado de nuestro decimal sobre 2.                          
    }                           //Además, incrementamos el valor de nuestro index.

    for(int i=index-1; i>=0; i--){
        printf("%d", binary[i]);    //Mostramos en pantalla desde el último elemento del index registrado en reversa
    }                               //el arreglo del binario

    printf("\n\n\n");
    return;             //retornamos para continuar el funcionamiento del código
}

int main(){
    int decimal; //Declaramos nuestro entero a usar para la conversion
    
    while(true){
        printf("Decimal to Binary\n|Enter| Enter Number   |Ctrl+C| Finish\nInsert your Decimal: ");
        scanf("%d", &decimal);  //Hacemos un menu donde se le indica al usuario los controles del menu.

        decimal_binary(decimal);    //Se llama la funcion de decimal a binario.
    }
}
