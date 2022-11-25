#ifndef UTIL_H_
#define UTIL_H_

#include <cstdlib>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

[[noreturn]] void die(const std::string_view reason) {
    std::cerr << reason << std::endl;
    std::exit(1);
}

std::string read_to_string(const std::string_view path) {
    std::ifstream ifs(path);
    std::stringstream ss;
    ss << ifs.rdbuf();
    if (!ifs.good()) die(strerror(errno));
    return ss.str();
}

template <typename T>
std::vector<T> parse_lines(const std::string_view path,
                           std::function<T(const std::string_view)> parse) {
    std::vector<T> lines;
    std::ifstream ifs(path);
    std::string line;
    while (std::getline(ifs, line)) {
        lines.push_back(parse(line));
    }
    if (!ifs.eof()) die(strerror(errno));
    return lines;
}

#endif  // UTIL_H_
