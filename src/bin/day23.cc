#include <array>
#include <cstdlib>
#include <deque>
#include <fstream>
#include <iostream>
#include <map>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

[[noreturn]] void die(const std::string& message) {
    std::cerr << message << std::endl;
    std::exit(1);
}

using pt = std::pair<int, int>;
using graph = std::map<pt, std::vector<pt>>;
using grid = std::vector<std::string>;

inline int cost(char c) {
    switch (c) {
        case 'A': return 1;
        case 'B': return 10;
        case 'C': return 100;
        case 'D': return 1000;
    }
}

inline int room(char c) {
    switch (c) {
        case 'A': return 0;
        case 'B': return 1;
        case 'C': return 2;
        case 'D': return 3;
    }
    die(std::string("bad room: ") + std::to_string(c));
}

inline char pod_for_room(int room) {
    switch (room) {
        case 0: return 'A';
        case 1: return 'B';
        case 2: return 'C';
        case 3: return 'D';
    }
    die("bad pod");
}

struct burrow {
    grid g;
    graph nbrs;
    std::vector<std::pair<char, pt>> pods;
    std::array<std::vector<pt>, 4> rooms;
};

inline bool in_hallway(const burrow& b, pt p) { return p.first == 1; }

inline bool in_front_of_room(const burrow& b, pt p) {
    return p.first == 1 && b.g[p.first - 1][p.second] == '#' &&
           b.g[p.first + 1][p.second] != '#';
}

inline bool own_room(const burrow& b, char pod, pt p) {
    return room(pod) == (p.second - 3) / 2;
}

inline bool all_own(const burrow& b, int room) {
    char pod = pod_for_room(room);
    for (auto [row, col] : b.rooms[room]) {
        if (char c = b.g[row][col]; c != '.' && c != pod) return false;
    }
    return true;
}

bool can_enter(const burrow& b, char pod, pt from, pt cur) {
    if (b.g[cur.first][cur.second] != '.') return false;
    if (in_hallway(b, from)) {
        if (in_hallway(b, cur)) return true;
        if (!own_room(b, pod, cur)) return false;
        if (!all_own(b, room(pod))) return false;
    } else {
        if (in_hallway(b, cur))
            return !own_room(b, pod, from) || !all_own(b, room(pod));
        if (own_room(b, pod, cur)) {
            return cur.second != from.second || cur.first > from.first ||
                   !all_own(b, room(pod));
        } else {
            return cur.first < from.first;
        }
    }
    return true;
}

bool can_stop(const burrow& b, char pod, pt from, pt cur) {
    if (in_hallway(b, from)) {
        return !in_hallway(b, cur) && own_room(b, pod, cur) &&
               all_own(b, int(pod));
    } else if (in_hallway(b, cur)) {
        return !in_front_of_room(b, cur);
    } else {
        return own_room(b, pod, cur);
    }
}

std::vector<pt> explore(const burrow& b, pt from) {
    std::vector<pt> pts;
    std::set<pt> v;
    v.insert(from);
    std::deque<pt> q;
    q.push_back(from);
    bool can_move_into_room = false;
    while (!q.empty()) {
        pt cur = q.front();
        q.pop_front();
        char pod = b.g[from.first][from.second];
        for (pt nbr : b.nbrs.at(cur)) {
            if (v.count(nbr)) continue;
            if (!can_enter(b, pod, from, nbr)) continue;
            if (can_stop(b, pod, from, nbr)) pts.push_back(nbr);
            q.push_back(nbr);
            v.insert(nbr);
        }
    }
    return pts;
}

burrow parse(std::istream&& is) {
    burrow b;
    for (std::string line; std::getline(is, line);) b.g.push_back(line);
    for (int row = 0; row < b.g.size(); row++) {
        for (int col = 0; col < b.g[row].size(); col++) {
            char c = b.g[row][col];
            if (c == '#' || c == ' ') continue;
            pt pos{row, col};
            if (row > 1 && (col == 3 || col == 5 || col == 7 || col == 9)) {
                b.rooms[(col - 3) / 2].push_back(pos);
            }
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
    for (int i = 0; i < 4; i++) {
        std::cout << "room " << i << ": ";
        for (pt p : b.rooms[i]) print(p);
        std::cout << std::endl;
    }
}

int main(int argc, char* argv[]) {
    if (argc != 2) die("usage: day23 <file>");
    burrow b = parse(std::ifstream(argv[1]));
    for (auto [pod, p] : b.pods) {
        std::cout << pod << " at ";
        print(p);
        std::cout << " -> ";
        for (pt nbr : explore(b, p)) print(nbr);
        std::cout << std::endl;
    }
}
