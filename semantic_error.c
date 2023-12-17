// This is a comment
float global = 1.5;
/*
* This is a multi-line comment
*/
int sum(int a) {
    int r = 0;
    while (a > 0) {
        r = r + a;
        a = a - 1;
    }
    int r = 0;
    return r;
}

int fact(int a) {
    if (a == 0) {
        return 1;
    }
    return a * fact(a - 1);
}

int fact(int a) {
    return 0;
}

char do_while(int a) {
    int r = 0;
    do {
        if (a - 5) {
            continue;
        }
        r = r + a;
        a = a - 1;
    } while (a > 0);
    break;
    return r;
}

int main(int argc) {
	int choice[2][0];
	choice[0][0] = 1;
	choice[0][1] = 2;
	choice[0][2] = 3;
	choice[1][0] = 'c';
	choice[1][1] = 20;
	choice[1][2] = 30;
	int i = 0;
	int j = 15;
	continue;
	i[0] = 1;
	choice[0] = 1;
	p = 1;
	char c = 'c';
	switch (choice[0][1]) {
        case 1:
            sum(choice[1][1], choice[1][2]);
            break;
        case 2:
            fact(c);
            break;
        default:
            j = 10;
            add(choice[1][1], choice[1][2]);
            break;
    }
	for (i = 0; i < true; i++) {
	    int j = 0;
        do_while(i);
    }
    char string[3] = "Hello";
    char strings[2][5] = "Hello";
    string[4] = '!';
    strings[4]++;
    return 0;
}