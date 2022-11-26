#include <iostream>
#include <vector>

#include "util.h"

enum class Tile { Right, Down, Empty };
enum class Dir { East, South };

using Grid = std::vector<std::vector<Tile>>;
using Pt = std::pair<int, int>;

struct Fish {
    Pt pt;
    Dir dir;
};

Grid read_grid(const std::string_view path) {
    auto parse_tile = [](char ch) {
        switch (ch) {
            case '>': return Tile::Right;
            case 'v': return Tile::Down;
            case '.': return Tile::Empty;
        }
        die("invalid tile");
    };
    auto parse_line = [=](const std::string_view line) {
        std::vector<Tile> tiles;
        for (char ch : line) tiles.push_back(parse_tile(ch));
        return tiles;
    };
    return parse_lines(path, std::function(parse_line));
}

Pt nbr_pt(const Grid& g, const Fish& fish) {
    const auto& [row, col] = fish.pt;
    if (fish.dir == Dir::South) return {(row + 1) % g.size(), col};
    if (fish.dir == Dir::East) return {row, (col + 1) % g[0].size()};
    die("empty can't move");
}

void step_herd(Grid& cur, Grid& next, std::vector<Fish>& fish, Dir dir) {
    for (auto& fish : fish) {
        if (fish.dir != dir) continue;
        auto& [row, col] = fish.pt;
        const auto& [nbr_row, nbr_col] = nbr_pt(cur, fish);
        if (cur[nbr_row][nbr_col] != Tile::Empty) continue;
        std::swap(next[row][col], next[nbr_row][nbr_col]);
        row = nbr_row;
        col = nbr_col;
    }
}

void step(Grid& cur, Grid& next, std::vector<Fish>& fish) {
    next = cur;
    step_herd(cur, next, fish, Dir::East);
    cur = next;
    step_herd(cur, next, fish, Dir::South);
    cur.swap(next);
}

std::vector<Fish> find_fish(const Grid& g) {
    std::vector<Fish> fish;
    for (int r = 0; r < g.size(); r++) {
        for (int c = 0; c < g[r].size(); c++) {
            switch (g[r][c]) {
                case Tile::Down: fish.push_back({{r, c}, Dir::South}); break;
                case Tile::Right: fish.push_back({{r, c}, Dir::East}); break;
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
