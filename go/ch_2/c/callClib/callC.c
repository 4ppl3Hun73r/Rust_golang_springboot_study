#include <stdio.h>
#include "callC.h"

void cHello() {
	printf("Hellow from C\n");
}

void printMessage(char* message) {
	printf("Go send me %s\n", message);
}
