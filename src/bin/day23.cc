#include <array>
#include <cstdlib>
#include <deque>
#include <fstream>
#include <iostream>
#include <limits>
#include <map>
#include <optional>
#include <set>
#include <sstream>
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
    die("bad cost");
}

inline int idx(char c) {
    switch (c) {
        case 'A': return 0;
        case 'B': return 1;
        case 'C': return 2;
        case 'D': return 3;
    }
    die("bad idx");
}

inline char pod_for_idx(int room) {
    switch (room) {
        case 0: return 'A';
        case 1: return 'B';
        case 2: return 'C';
        case 3: return 'D';
    }
    die("bad pod: " + std::to_string(room));
}

struct burrow {
    grid g;
    graph nbrs;
    std::array<std::set<pt>, 4> pods;
    std::array<std::vector<pt>, 4> rooms;
};

inline bool in_hallway(const burrow& b, pt p) { return p.first == 1; }

inline bool in_front_of_room(const burrow& b, pt p) {
    return p.first == 1 && b.g[p.first - 1][p.second] == '#' &&
           b.g[p.first + 1][p.second] != '#';
}

inline bool own_room(const burrow& b, char pod, pt p) {
    return idx(pod) == (p.second - 3) / 2;
}

inline bool all_own(const burrow& b, int room) {
    char pod = pod_for_idx(room);
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
        if (!all_own(b, idx(pod))) return false;
    } else {
        if (in_hallway(b, cur))
            return !own_room(b, pod, from) || !all_own(b, idx(pod));
        if (own_room(b, pod, cur)) {
            return cur.second != from.second || cur.first > from.first ||
                   !all_own(b, idx(pod));
        } else {
            return cur.first < from.first;
        }
    }
    return true;
}

bool can_stop(const burrow& b, char pod, pt from, pt cur) {
    if (in_hallway(b, from)) {
        return !in_hallway(b, cur) && own_room(b, pod, cur) &&
               all_own(b, idx(pod));
    } else if (in_hallway(b, cur)) {
        return !in_front_of_room(b, cur);
    } else {
        return own_room(b, pod, cur);
    }
}

using edge = std::pair<pt, int>;
std::vector<edge> explore(const burrow& b, pt from) {
    std::vector<edge> edges;
    std::set<pt> v;
    v.insert(from);
    std::deque<edge> q;
    q.push_back({from, 0});
    bool can_move_into_room = false;
    while (!q.empty()) {
        auto [cur, dist] = q.front();
        q.pop_front();
        char pod = b.g[from.first][from.second];
        for (pt nbr : b.nbrs.at(cur)) {
            if (v.count(nbr)) continue;
            if (!can_enter(b, pod, from, nbr)) continue;
            if (can_stop(b, pod, from, nbr)) edges.push_back({nbr, dist + 1});
            q.push_back({nbr, dist + 1});
            v.insert(nbr);
        }
    }
    return edges;
}

void print(std::ostream& os, pt p) {
    os << '[' << p.first << ',' << p.second << ']';
}

std::string key(const burrow& b) {
    std::stringstream s;
    for (int i = 0; i < 4; i++) {
        for (pt p : b.pods[i]) print(s, p);
        s << '|';
    }
    return s.str();
}

bool done(const burrow& b) {
    for (int i = 0; i < 4; i++) {
        for (pt p : b.pods[i]) {
            if (in_hallway(b, p) || !own_room(b, pod_for_idx(i), p))
                return false;
        }
    }
    return true;
}

constexpr int kMaxInt = std::numeric_limits<int>::max();

int min_energy(burrow& b, std::map<std::string, int>& memo) {
    if (done(b)) return 0;
    auto k = key(b);
    if (auto it = memo.find(k); it != memo.end()) return it->second;
    int max = kMaxInt;
    std::vector<grid> path;
    for (int i = 0; i < 4; i++) {
        auto ps = b.pods[i];
        for (pt p : ps) {
            auto moves = explore(b, p);
            // TODO: prefer moving to final spot
            for (auto [q, d] : moves) {
                int energy = d * cost(pod_for_idx(i));
                // TODO: estimate remaining cost

                std::swap(b.g[p.first][p.second], b.g[q.first][q.second]);
                b.pods[i].erase(p);
                b.pods[i].insert(q);
                auto rest = min_energy(b, memo);
                std::swap(b.g[p.first][p.second], b.g[q.first][q.second]);
                b.pods[i].erase(q);
                b.pods[i].insert(p);

                if (rest == kMaxInt) continue;
                max = std::min(max, rest + energy);
            }
        }
    }
    return memo[k] = max;
}

int min_energy(burrow& b) {
    std::map<std::string, int> memo;
    return min_energy(b, memo);
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
            if (std::isalpha(c)) b.pods[idx(c)].insert(pos);
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

int main(int argc, char* argv[]) {
    if (argc != 2) die("usage: day23 <file>");
    burrow b = parse(std::ifstream(argv[1]));
    std::cout << min_energy(b) << std::endl;
}
