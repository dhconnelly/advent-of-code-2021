#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
    INP,
    ADD,
    MUL,
    DIV,
    MOD,
    EQL,
} opcode;

opcode parse_opcode(char* line) {
    if (strcmp(line, "inp") == 0) {
        return INP;
    } else if (strcmp(line, "add") == 0) {
        return ADD;
    } else if (strcmp(line, "mul") == 0) {
        return MUL;
    } else if (strcmp(line, "mod") == 0) {
        return MOD;
    } else if (strcmp(line, "div") == 0) {
        return DIV;
    } else if (strcmp(line, "eql") == 0) {
        return EQL;
    } else {
        assert(0);
    }
}

const char* str_of_opcode(opcode op) {
    switch (op) {
        case INP:
            return "inp";
        case ADD:
            return "add";
        case MUL:
            return "mul";
        case DIV:
            return "div";
        case MOD:
            return "mod";
        case EQL:
            return "eql";
    }
}

typedef enum {
    VAR,
    NUM,
} param_type;

typedef struct {
    param_type type;
    union {
        char var;
        int num;
    } data;
} param;

param parse_param(char* line) {
    param p;
    if ((line[0] >= '0' && line[0] <= '9') || line[0] == '-') {
        p.type = NUM;
        p.data.num = atoi(line);
        return p;
    } else {
        p.type = VAR;
        p.data.var = *line;
    }
    return p;
}

typedef struct {
    opcode op;
    char arg1;
    param arg2;
} instr;

void parse_instr(instr* i, char* line) {
    /* parse opcode */
    char* sep = strstr(line, " ");
    assert(sep != NULL);
    *sep = '\0';
    i->op = parse_opcode(line);

    /* parse arg1 */
    line = sep + 1;
    sep = strstr(line, " ");
    if (sep != NULL) *sep = '\0';
    i->arg1 = line[0];

    /* parse arg2 if present */
    if (sep != NULL) {
        line = sep + 1;
        i->arg2 = parse_param(line);
    }
}

void print_instr(instr i) {
    printf("%s %c", str_of_opcode(i.op), i.arg1);
    if (i.op != INP) {
        switch (i.arg2.type) {
            case VAR:
                printf(" %c", i.arg2.data.var);
                break;
            case NUM:
                printf(" %d", i.arg2.data.num);
                break;
        }
    }
    printf("\n");
}

static const int MAXLINE = 32;
static const int MAXINSTRS = 1024;

int read_instrs(instr instrs[], FILE* input) {
    char line[MAXLINE];
    int i;
    for (i = 0; i < MAXINSTRS && fgets(line, MAXLINE, input) != NULL; i++) {
        if (line[0] == '\n' || line[0] == '\0') break;
        parse_instr(&instrs[i], line);
    }
    return i;
}

int main(int argc, char* argv[]) {
    if (argc != 2) {
        printf("usage: day24 <filename>\n");
        exit(1);
    }

    FILE* input = fopen(argv[1], "r");
    if (input == NULL) {
        perror("can't open input file");
        exit(1);
    }

    instr instrs[MAXINSTRS];
    int num_instrs = read_instrs(instrs, input);

    int i;
    for (i = num_instrs - 1; i >= 0; i--) {
        print_instr(instrs[i]);
    }

    return 0;
}
