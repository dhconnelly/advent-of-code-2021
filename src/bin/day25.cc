#include <iostream>
#include <vector>

#include "util.h"

enum class Tile {
    Right,
    Down,
    Empty,
};

using Grid = std::vector<std::vector<Tile>>;

Tile parse_tile(char ch) {
    switch (ch) {
        case '>': return Tile::Right;
        case 'v': return Tile::Down;
        case '.': return Tile::Empty;
    }
    die("invalid tile");
}

Grid read_grid(const std::string_view path) {
    return parse_lines(
        path, std::function([](std::string_view line) -> std::vector<Tile> {
            std::vector<Tile> tiles;
            for (char ch : line) tiles.push_back(parse_tile(ch));
            return tiles;
        }));
}

std::ostream& operator<<(std::ostream& os, const Grid& g) {
    for (const auto& row : g) {
        for (const auto& col : row) {
            std::cout << (col == Tile::Down    ? 'v'
                          : col == Tile::Right ? '>'
                                               : '.');
        }
        std::cout << '\n';
    }
    return std::cout;
}

using pt = std::pair<int, int>;

enum class Dir {
    East,
    South,
};

pt nbr(pt cur, Dir dir, const Grid& g) {
    if (dir == Dir::South) return {(cur.first + 1) % g.size(), cur.second};
    if (dir == Dir::East) return {cur.first, (cur.second + 1) % g[0].size()};
    die("empty can't move");
}

struct Fish {
    std::vector<pt> east;
    std::vector<pt> south;
};

void step_herd(Grid& cur, Grid& next, std::vector<pt>& fish, Dir dir) {
    for (auto& [row, col] : fish) {
        Tile cur_tile = cur[row][col];
        const auto& [nbr_row, nbr_col] = nbr({row, col}, dir, cur);
        if (cur[nbr_row][nbr_col] != Tile::Empty) continue;
        next[row][col] = Tile::Empty;
        next[nbr_row][nbr_col] = cur_tile;
        row = nbr_row;
        col = nbr_col;
    }
}

void step(Grid& cur, Grid& next, Fish& fish) {
    next = cur;
    step_herd(cur, next, fish.east, Dir::East);
    cur = next;
    step_herd(cur, next, fish.south, Dir::South);
    cur = next;
}

Fish find_fish(const Grid& g) {
    Fish fish;
    for (int row = 0; row < g.size(); row++) {
        for (int col = 0; col < g[row].size(); col++) {
            switch (g[row][col]) {
                case Tile::Down: fish.south.emplace_back(row, col); break;
                case Tile::Right: fish.east.emplace_back(row, col); break;
                case Tile::Empty: break;
            }
        }
    }
    return fish;
}

int main(int argc, char* argv[]) {
    if (argc != 2) die("usage: day25 <input>");
    auto g = read_grid(argv[1]);
    auto tmp = g;
    auto fish = find_fish(g);
    for (int i = 1;; i++) {
        auto snapshot = g;
        step(g, tmp, fish);
        if (g == snapshot) {
            std::cout << i << std::endl;
            break;
        }
    }
    return 0;
}
