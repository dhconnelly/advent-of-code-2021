#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>
#include <optional>
#include <string>
#include <utility>
#include <vector>

[[noreturn]] void die(const char* message) {
    std::cerr << message << std::endl;
    std::exit(1);
}

using pt = std::pair<int, int>;
using graph = std::map<pt, std::vector<pt>>;
using grid = std::vector<std::string>;

struct burrow {
    grid g;
    graph nbrs;
    std::vector<std::pair<char, pt>> pods;
};

burrow parse(std::istream&& is) {
    burrow b;
    for (std::string line; std::getline(is, line);) b.g.push_back(line);
    for (int row = 0; row < b.g.size(); row++) {
        for (int col = 0; col < b.g[row].size(); col++) {
            char c = b.g[row][col];
            if (c == '#' || c == ' ') continue;
            pt pos{row, col};
            if (std::isalpha(c)) b.pods.emplace_back(c, pos);
            static constexpr pt kDirs[] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
            for (int i = 0; i < 4; i++) {
                pt nbr{row + kDirs[i].first, col + kDirs[i].second};
                // conveniently the edge cases don't appear...
                if (b.g[nbr.first][nbr.second] == '#') continue;
                b.nbrs[pos].push_back(nbr);
            }
        }
    }
    return b;
}

void print(pt p) { std::cout << '[' << p.first << ',' << p.second << ']'; }

void print(const burrow& b) {
    for (const auto& row : b.g) std::cout << row << std::endl;
    for (const auto& [p, nbrs] : b.nbrs) {
        print(p);
        std::cout << " -> ";
        for (pt nbr : nbrs) print(nbr);
        std::cout << std::endl;
    }
    for (const auto& [pod, p] : b.pods) {
        std::cout << pod << ": ";
        print(p);
        std::cout << std::endl;
    }
}

int main(int argc, char* argv[]) {
    if (argc != 2) die("usage: day23 <file>");
    burrow b = parse(std::ifstream(argv[1]));
    print(b);
}
