#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

static const int AMPS[4] = {A, B, C, D};
static const int AMP_COLS[4] = {2, 4, 6, 8};
static const int64_t AMP_ENERGY[4] = {1, 10, 100, 1000};

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

/***********************************/
/* k&r hash table                  */

#define HASHSIZE 101

unsigned hash(char* s) {
    unsigned hashval;
    for (hashval = 0; *s != '\0'; s++) hashval = *s + 31 * hashval;
    return hashval % HASHSIZE;
}

typedef struct nlist {
    struct nlist* next;
    char* key;
    int64_t value;
} nlist;

typedef nlist* hashset[HASHSIZE];

int64_t* lookup(hashset set, char* key) {
    nlist* np;
    for (np = set[hash(key)]; np != NULL; np = np->next) {
        if (strcmp(key, np->key) == 0) return &np->value;
    }
    return NULL;
}

void insert(hashset set, char* key, int64_t value) {
    nlist* np = malloc(sizeof(nlist));
    unsigned hashval = hash(key);
    np->next = set[hashval];
    set[hashval] = np;
    np->key = strdup(key);
    np->value = value;
}

/***********************************/

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
            int col = AMP_COLS[i] + 1;
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
            int col = AMP_COLS[j];
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

int room_all_of_type(state* st, int amp_idx) {
    amp want = AMPS[amp_idx];
    int depth;
    for (depth = 0; depth < st->room_depth; depth++) {
        amp present = st->rooms[amp_idx][depth];
        if (present != NONE && present != want) {
            return 0;
        }
    }
    return 1;
}

int col_of(pos p) { return (p.where == -1) ? p.idx : AMP_COLS[p.where]; }

int64_t shortest_path(state* st, pos cur, pos dest) {
    int64_t dist = 0;

    /* in a room? first move to hallway */
    if (cur.where >= 0) {
        int row;
        for (row = cur.idx; row >= 0; row -= 1) {
            if (row > 0 && st->rooms[cur.where][row - 1] != NONE) return -1;
            dist += 1;
        }
    }

    /* in hallway: move to target column */
    int cur_col = col_of(cur);
    int dest_col = col_of(dest);
    int col_dir = (dest_col - cur_col) > 0 ? 1 : -1;
    int col;
    for (col = cur_col; col != dest_col; col += col_dir) {
        if (st->hallway[col + col_dir] != NONE) return -1;
        dist += 1;
    }

    /* going to a room? move to target row */
    if (dest.where >= 0) {
        int row;
        for (row = -1; row < dest.idx; row += 1) {
            if (st->rooms[dest.where][row + 1] != NONE) return -1;
            dist += 1;
        }
    }

    return dist;
}

void set(state* st, pos p, amp amp) {
    if (p.where >= 0) {
        st->rooms[p.where][p.idx] = amp;
    } else {
        st->hallway[p.idx] = amp;
    }
}

int64_t minimize(state*);

int64_t minimize_from(state* st, int amp_idx, int amp_depth, pos dest, int dist,
                      int64_t min_so_far) {
    pos cur = st->amps[amp_idx][amp_depth];
    st->amps[amp_idx][amp_depth] = dest;
    set(st, cur, NONE);
    set(st, dest, AMPS[amp_idx]);

    int64_t energy_cost = AMP_ENERGY[amp_idx] * dist;
    int64_t recursive_cost = minimize(st);
    int64_t total_cost = energy_cost + recursive_cost;
    if (recursive_cost >= 0 && total_cost < min_so_far) {
        min_so_far = total_cost;
    }

    set(st, cur, AMPS[amp_idx]);
    set(st, dest, NONE);
    st->amps[amp_idx][amp_depth] = cur;
    return min_so_far;
}

hashset global_set;
static const int KEY_LEN = 7 * 8 + 1;
typedef char state_key[KEY_LEN];

void k(state* st, state_key key) {
    int i = 0, amp_idx, amp_depth;
    for (amp_idx = 0; amp_idx < NUM_AMPS; amp_idx++) {
        for (amp_depth = 0; amp_depth < st->room_depth; amp_depth++) {
            pos p = st->amps[amp_idx][amp_depth];
            i += sprintf(key + i, "[%02d,%02d]", p.where, p.idx);
        }
    }
    key[KEY_LEN - 1] = '\0';
}

int is_amp_col(int cur_col) {
    int i;
    for (i = 0; i < NUM_AMPS; i++) {
        if (cur_col == AMP_COLS[i]) return 1;
    }
    return 0;
}

int64_t minimize(state* st) {
    char key[KEY_LEN];
    k(st, key);
    int64_t* cached;
    if ((cached = lookup(global_set, key)) != NULL) return *cached;

    int in_place = 0;
    int64_t min_cost = INT64_MAX;
    int amp_idx, amp_depth;
    for (amp_idx = 0; amp_idx < NUM_AMPS; amp_idx++) {
        for (amp_depth = 0; amp_depth < st->room_depth; amp_depth++) {
            pos cur = st->amps[amp_idx][amp_depth];

            int can_go_home = room_all_of_type(st, amp_idx);
            if (cur.where == amp_idx && can_go_home) {
                /* in its own room and can stay there. do nothing */
                /* invariant: this implies it's as deep as it can go */
                in_place++;
                continue;
            }

            /* TODO: we can know the deepest tile without searching */
            int dist;
            if (can_go_home) {
                /* go as deep as we can */
                int depth;
                for (depth = st->room_depth - 1; depth >= 0; depth--) {
                    pos dest = {amp_idx, depth};
                    if ((dist = shortest_path(st, cur, dest)) > 0) {
                        min_cost = minimize_from(st, amp_idx, amp_depth, dest,
                                                 dist, min_cost);
                        break;
                    }
                }
            }

            if (cur.where != -1) {
                /* go left and right until hitting an obstacle */
                int cur_col = col_of(cur);
                int dir;
                for (dir = -1; dir <= 1; dir += 2) {
                    int col;
                    for (col = cur_col + dir; col >= 0 && col < HALLWAY_LEN;
                         col += dir) {
                        if (is_amp_col(col)) continue;
                        pos dest = {-1, col};
                        int dist = shortest_path(st, cur, dest);
                        if (dist < 0) break;
                        min_cost = minimize_from(st, amp_idx, amp_depth, dest,
                                                 dist, min_cost);
                    }
                }
            }
        }
    }

    int done = in_place == 4 * st->room_depth;
    int64_t cost = done ? 0 : min_cost < INT64_MAX ? min_cost : -1;
    insert(global_set, key, cost);
    return cost;
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

    printf("%lld\n", minimize(&st));

    return 0;
}
