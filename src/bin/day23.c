#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

/* static const int ROOM_COLS[4] = {2, 4, 6, 8}; */
static const int HALLWAY_LEN = 11;
static const int NUM_AMPS = 4;
/* line len = hallway plus wall on each side, newline, NUL */
static const int LINE_LEN = 15;

typedef enum {
    NONE = '.',
    A = 'A',
    B = 'B',
    C = 'C',
    D = 'D',
} amp;

static const int COLS[4] = {2, 4, 6, 8};

typedef struct {
    int where; /* -1 for hallway, 0.. for room index */
    int idx;
} pos;

typedef struct {
    /* for each of the NUM_AMPS amp types there are room_depth amps */
    size_t room_depth;
    amp hallway[HALLWAY_LEN];
    /* each of the NUM_AMPS rooms[N] has size room_depth */
    amp* rooms[NUM_AMPS];
    /* amps[N] has the room_depth positions for amp type N */
    pos* amps[NUM_AMPS];
} state;

state make_state(int room_depth) {
    state st;
    st.room_depth = room_depth;
    int i, j;
    for (i = 0; i < NUM_AMPS; i++) {
        st.rooms[i] = malloc(room_depth * sizeof(amp));
        for (j = 0; j < room_depth; j++) st.rooms[i][j] = NONE;
    }
    for (i = 0; i < NUM_AMPS; i++) {
        st.amps[i] = malloc(room_depth * sizeof(pos));
        /* positions to be filled later */
    }
    for (i = 0; i < HALLWAY_LEN; i++) {
        st.hallway[i] = NONE;
    }
    return st;
}

void print_state(state* st) {
    /* top wall */
    int i, line_len = HALLWAY_LEN + 2;
    for (i = 0; i < line_len; i++) putchar('#');
    putchar('\n');

    /* hallway */
    putchar('#');
    for (i = 0; i < HALLWAY_LEN; i++) putchar(st->hallway[i]);
    putchar('#');
    putchar('\n');

    /* rooms */
    int depth;
    for (depth = 0; depth < st->room_depth; depth++) {
        char line[HALLWAY_LEN + 2];
        for (i = 0; i < HALLWAY_LEN + 2; i++) {
            line[i] = '#';
        }
        for (i = 0; i < NUM_AMPS; i++) {
            int col = COLS[i] + 1;
            amp amp = st->rooms[i][depth];
            line[col] = amp;
        }
        printf("%s\n", line);
    }

    /* bottom wall */
    for (i = 0; i < line_len; i++) putchar('#');
    putchar('\n');
}

void parse_state(state* st, FILE* input) {
    char line[LINE_LEN];
    /* top wall */
    assert(fgets(line, LINE_LEN, input) != NULL);

    /* hallway */
    assert(fgets(line, LINE_LEN, input) != NULL);

    /* input always has depth 2 */
    int room_depth = 2;
    int i, j;
    int amp_count[NUM_AMPS];
    for (i = 0; i < NUM_AMPS; i++) {
        amp_count[i] = 0;
    }
    for (i = 0; i < room_depth; i++) {
        assert(fgets(line, LINE_LEN, input) != NULL);
        for (j = 0; j < NUM_AMPS; j++) {
            int col = COLS[j];
            amp amp = line[1 + col];
            int amp_idx = amp - A;
            st->rooms[j][i] = amp;
            int n = amp_count[amp_idx];
            st->amps[amp_idx][n].where = j;
            st->amps[amp_idx][n].idx = i;
            amp_count[amp_idx]++;
        }
    }

    /* bottom wall */
    assert(fgets(line, LINE_LEN, input) != NULL);
}

int main(int argc, char* argv[]) {
    if (argc != 2) {
        printf("usage: day23 <file>\n");
        exit(1);
    }

    FILE* input = fopen(argv[1], "r");
    if (input == NULL) {
        perror("can't open input file");
        exit(1);
    }

    state st = make_state(2);
    parse_state(&st, input);
    print_state(&st);

    return 0;
}
