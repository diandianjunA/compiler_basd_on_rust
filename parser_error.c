// This is a comment
float global = 1.5;
/*
* This is a multi-line comment
*/
int sum(int a) {
    int r = 0;
    while (a > 0 && r > 0) {
        r = 5
        a = a - 1;
    }
    return r;
}

int fact(int a) {
    if (a == 0) {
        return 1;
    }
    return a * fact(a - 1);
}

int do_while(int a) {
    int r = 0;
    do {
        if (a == 5) {
            continue;
        }
        r = 5 + 1;
        a = a - 1;
    } while (a > 0);
    return r;
}

int main(void) {
	int choice[2][3];
	choice[0][0] = 1;
	choice[0][1] = 2;
	choice[0][2] = 3;
	choice[1][0] = 10;
	choice[1][1] = 20;
	choice[1][2] = 30;
	int i = 0;
	int j = 15;
	int k = 0;
	if (k != 0) {
	    k = 1;
	}
	switch (choice[0][1]) {
        case 1:
            sum(choice[1][1]);
            break;
        case 2:
            fact(choice[1][2]);
            break;
        default:
            j = 10;
            break;
    }
	for (i = 0; i < j; i++) {
	    k = 1;
        do_while(i);
    }
    char string[10] = "Hello";
    string[4] = '!';
    return 0;
}
