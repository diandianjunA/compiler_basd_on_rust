// This is a comment
float global_variable = 1.5;

int function_add(int a, int b) {
    int c = a + b;
    return c;
}

/*
* This is a multi-line comment
*/
int function_sum(int a) {
    int b = 0;
    int i = 0;
    global_variable = 0.0;
    for (i = 0; i < a; i++) {
        b = function_add(b, i);
    }
    return b;
}

int function_void(void) {
    return 1;
}

int main(void) {
	int variable = 0;
	int choice[2][2];
	choice[0][0] = 1;
	choice[0][1] = 2;
	variable = function_void();
	switch (choice[0][1]) {
	    case 1:
            variable = 1;
            break;
        case 2:
            variable = 2;
            break;
        default:
            variable = 0;
            break;
    }
    int a = 0;
    int b = 0;
    while (a < 10) {
        a++;
        int f = 0;
        b = function_add(a, b);
        if (a == 5) {
            continue;
        }
        if (a == 7 && b == 1) {
            break;
        }
    }
    int e = function_sum(10);
    int f = 0;
    int i = 0;
    for (i = 0; i < 10; i++) {
        f = function_add(f, i);
        int g = 0;
	}
	char string[10] = "Hello";
	string[5] = '5';
	return 0;
}
