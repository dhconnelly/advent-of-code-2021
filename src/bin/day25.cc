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

struct Fish {
    std::vector<pt> east;
    std::vector<pt> south;
};

void step(Grid& g, Grid& tmp, Fish& fish) {}

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
    std::cout << "east:\n";
    for (const auto& [row, col] : fish.east)
        std::cout << row << ',' << col << '\n';
    std::cout << "south:\n";
    for (const auto& [row, col] : fish.south)
        std::cout << row << ',' << col << '\n';
    std::cout << g;
    for (int i = 1; i <= 4; i++) {
        std::cout << "After " << i << " steps:\n";
        step(g, tmp, fish);
        std::cout << g;
    }
    return 0;
}
