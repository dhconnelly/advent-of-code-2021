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

int main(int argc, char* argv[]) {
    if (argc != 2) die("usage: day25 <input>");
    auto g = read_grid(argv[1]);
    for (const auto& row : g) {
        for (const auto& col : row) {
            std::cout << (col == Tile::Down    ? 'v'
                          : col == Tile::Right ? '>'
                                               : '.');
        }
        std::cout << '\n';
    }
    return 0;
}
