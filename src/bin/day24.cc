#include <algorithm>
#include <array>
#include <cassert>
#include <cerrno>
#include <charconv>
#include <cstdlib>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <iterator>
#include <ranges>
#include <regex>
#include <sstream>
#include <unordered_map>
#include <variant>

// ============================================================================
// general utilities

[[noreturn]] void die(const std::string_view msg) {
    std::cerr << msg << '\n';
    std::exit(1);
}

std::string read_file(const std::filesystem::path& path) {
    std::ifstream ifs{path};
    if (!ifs.good()) die(strerror(errno));
    std::stringstream ss;
    ss << ifs.rdbuf();
    return ss.str();
}

// ============================================================================
// model

enum class opcode : int8_t { Inp = 0, Add, Mul, Div, Mod, Eql };
constexpr std::string_view opcode_names[] = {
    "inp", "add", "mul", "div", "mod", "eql",
};
opcode parse_opcode(const std::string_view name) {
    auto it = std::ranges::find(opcode_names, name);
    if (it == std::end(opcode_names)) die("bad opcode: " + std::string(name));
    return static_cast<opcode>(it - std::begin(opcode_names));
}
std::string_view opcode_name(opcode op) {
    return opcode_names[static_cast<int>(op)];
}

constexpr bool binary_op[] = {false, true, true, true, true, true};
bool is_binary(opcode op) { return binary_op[static_cast<int>(op)]; }

using var = int8_t;
var parse_var(const std::string_view name) {
    char ch = name[0];
    assert(ch >= 'w' && ch <= 'z');
    return ch - 'w';
}
char var_name(var v) { return 'w' + v; }

enum class arg_type : int8_t { Var, Lit };
struct arg {
    int64_t val;
    arg_type typ;
};
arg parse_arg(const std::string& name) {
    if (std::isalpha(name[0])) return {parse_var(name), arg_type::Var};
    return {.val = std::stoi(name), .typ = arg_type::Lit};
}

struct instr {
    arg arg;
    var dest;
    opcode op;
};

using state = std::array<int64_t, 4>;
int64_t get(const state& state, const char* var) {
    return state[parse_var(var)];
}

// ============================================================================
// serialization

std::ostream& operator<<(std::ostream& os, const instr& instr) {
    os << opcode_name(instr.op) << ' ';
    os << var_name(instr.dest) << ' ';
    if (is_binary(instr.op)) {
        if (instr.arg.typ == arg_type::Lit)
            os << instr.arg.val;
        else
            os << var_name(instr.arg.val);
    }
    return os;
}

std::vector<instr> parse(const std::string& text) {
    static std::regex empty(R"(^[ ]*$)");
    static std::regex pat(R"(^[ ]*(\w+) (\w)[ ]*(\w|(-?\d+))?$)");
    std::vector<instr> prog;
    std::string line;
    std::smatch match;
    for (std::stringstream ss(text); std::getline(ss, line);) {
        if (std::regex_match(line, match, empty)) continue;
        if (!std::regex_match(line, match, pat)) die("bad line: " + line);
        auto& instr = prog.emplace_back();
        instr.op = parse_opcode(match[1].str());
        instr.dest = parse_var(match[2].str());
        if (match[3].length() > 0) instr.arg = parse_arg(match[3].str());
    }
    return prog;
}

// ============================================================================
// execution

class ALU {
   public:
    ALU(const std::vector<instr>& prog, const std::vector<int64_t>& inputs)
        : prog_(prog), inputs_(inputs) {}

    void step();
    bool done() { return pc_ >= prog_.size(); }
    static state take(ALU&& alu) { return alu.state_; }

    int64_t get(var var) { return state_[var]; }
    int64_t get(arg arg) {
        return (arg.typ == arg_type::Var) ? get(arg.val) : arg.val;
    }

    instr pop_instr() {
        if (pc_ >= prog_.size()) die("end of program");
        return prog_[pc_++];
    }

    int64_t pop_input() {
        if (inp_ >= inputs_.size()) die("no more inputs");
        return inputs_[inp_++];
    }

   private:
    state state_ = {0, 0, 0, 0};
    int pc_ = 0, inp_ = 0;
    std::vector<instr> prog_;
    std::vector<int64_t> inputs_;
};

void ALU::step() {
    instr instr = pop_instr();
    if (getenv("TRACE")) {
        std::cout << instr << '\n';
    }
    switch (instr.op) {
        case opcode::Inp:
            state_[instr.dest] = pop_input();
            break;
        case opcode::Add:
            state_[instr.dest] += get(instr.arg);
            break;
        case opcode::Mul:
            state_[instr.dest] *= get(instr.arg);
            break;
        case opcode::Div:
            state_[instr.dest] /= get(instr.arg);
            break;
        case opcode::Mod:
            state_[instr.dest] %= get(instr.arg);
            break;
        case opcode::Eql:
            state_[instr.dest] = (get(instr.dest) == get(instr.arg));
            break;
    }
    if (getenv("TRACE")) {
        for (int i = 0; i < 4; i++) std::cout << state_[i];
        std::cout << '\n';
    }
}

state run(const std::vector<instr>& prog, const std::vector<int64_t>& inputs) {
    ALU alu(prog, inputs);
    while (!alu.done()) alu.step();
    return ALU::take(std::move(alu));
}

state run(std::string text, std::vector<int64_t> inputs) {
    auto prog = parse(text);
    return run(prog, inputs);
}

// ============================================================================
// optimization

struct expr {
    virtual ~expr() {}
    virtual std::string str() const = 0;
    virtual std::unique_ptr<expr> eval(
        const std::unordered_map<char, std::unique_ptr<expr>>& es) const = 0;
    virtual std::unique_ptr<expr> clone() const = 0;
};

using exprs = std::unordered_map<char, std::unique_ptr<expr>>;

struct var_expr : public expr {
    var_expr(char var) : var(var) {}
    char var;
    std::string str() const override { return std::string(1, var); }
    std::unique_ptr<expr> clone() const override {
        return std::make_unique<var_expr>(var);
    }
    std::unique_ptr<expr> eval(const exprs& es) const override {
        return es.at(var)->clone();
    }
};

struct lit_expr : public expr {
    lit_expr(int64_t value) : value(value) {}
    int64_t value;
    std::string str() const override { return std::to_string(value); }
    std::unique_ptr<expr> clone() const override {
        return std::make_unique<lit_expr>(value);
    }
    std::unique_ptr<expr> eval(const exprs& es) const override {
        return clone();
    };
};

struct inp_expr : public expr {
    static int next_index;
    int index;
    inp_expr() : index(next_index++) {}
    inp_expr(int index) : index(next_index) {}
    std::string str() const override {
        return "input" + std::to_string(index) + "()";
    }
    std::unique_ptr<expr> clone() const override {
        return std::make_unique<inp_expr>(index);
    }
    std::unique_ptr<expr> eval(const exprs& es) const override {
        return clone();
    };
};

int inp_expr::next_index = 0;

struct bin_expr : public expr {
    bin_expr(char op, std::unique_ptr<expr> left, std::unique_ptr<expr> right)
        : op(op), left(std::move(left)), right(std::move(right)) {}
    char op;
    std::unique_ptr<expr> left, right;
    std::string str() const override {
        return '(' + left->str() + op + right->str() + ')';
    }
    std::unique_ptr<expr> clone() const override {
        return std::make_unique<bin_expr>(op, left->clone(), right->clone());
    }
    std::unique_ptr<expr> eval(const exprs& es) const override {
        return std::make_unique<bin_expr>(op, left->eval(es), right->eval(es));
    };
};

struct stmt {
    char lhs;
    std::unique_ptr<expr> rhs;
};

char to_op(opcode op) {
    switch (op) {
        case opcode::Add:
            return '+';
        case opcode::Mul:
            return '*';
        case opcode::Div:
            return '/';
        case opcode::Mod:
            return '%';
        case opcode::Eql:
            return '=';
        default:
            die("invalid op");
    }
}

std::unique_ptr<expr> to_expr(const arg& arg) {
    if (arg.typ == arg_type::Lit) {
        return std::make_unique<lit_expr>(arg.val);
    } else {
        return std::make_unique<var_expr>(var_name(arg.val));
    }
}

std::unique_ptr<expr> to_expr(const instr& instr) {
    switch (instr.op) {
        case opcode::Inp:
            return std::make_unique<inp_expr>();
        case opcode::Add:
        case opcode::Mul:
        case opcode::Div:
        case opcode::Mod:
        case opcode::Eql:
            return std::make_unique<bin_expr>(
                to_op(instr.op),
                std::make_unique<var_expr>(var_name(instr.dest)),
                to_expr(instr.arg));
    }
}

stmt to_stmt(const instr& instr) {
    return {var_name(instr.dest), to_expr(instr)};
}

std::vector<stmt> decompile1(const std::vector<instr>& prog) {
    std::vector<stmt> result;
    std::ranges::transform(prog, std::back_inserter(result), to_stmt);
    return result;
}

std::unique_ptr<expr> eval(const expr& e, const exprs& es) {
    return e.eval(es);
}

std::ostream& operator<<(std::ostream& os, const expr& e) {
    return os << e.str();
}

std::ostream& operator<<(std::ostream& os, const stmt& s) {
    return os << s.lhs << " <- " << *s.rhs;
}

exprs decompile(const std::vector<instr>& prog) {
    exprs es;
    es['w'] = std::make_unique<lit_expr>(0);
    es['x'] = std::make_unique<lit_expr>(0);
    es['y'] = std::make_unique<lit_expr>(0);
    es['z'] = std::make_unique<lit_expr>(0);
    for (const auto& stmt : decompile1(prog)) {
        auto e = eval(*stmt.rhs, es);
        std::cout << "applying: " << stmt << '\n';
        es[stmt.lhs] = std::move(e);
    }
    return es;
}

// ============================================================================
// tests

void run_tests() {
    {
        auto prog = R"(
            inp x
            mul x -1
        )";
        state result = run(prog, {7});
        assert(get(result, "x") == -7);
    }

    {
        auto prog = R"(
            inp z
            inp x
            mul z 3
            eql z x
        )";
        state result1 = run(prog, {8, 3});
        assert(get(result1, "z") == 0);
        state result2 = run(prog, {3, 9});
        assert(get(result2, "z") == 1);
    }

    {
        auto prog = R"(
            inp w
            add z w
            mod z 2
            div w 2
            add y w
            mod y 2
            div w 2
            add x w
            mod x 2
            div w 2
            mod w 2
        )";
        state result = run(prog, {/*11010*/ 26});
        state expected{1, 0, 1, 0};
        assert(result == expected);

        for (const auto& [var, expr] : decompile(parse(prog)))
            std::cout << var << '=' << *expr << '\n';
    }
}

void solve(const std::string& input) {
    auto prog = parse(input);
    auto exprs = decompile(prog);
    for (const auto& [var, expr] : exprs) {
        std::cout << var << '=' << *expr << '\n';
    }
}

int main(int argc, char* argv[]) {
    if (std::getenv("TEST"))
        run_tests();
    else if (argc != 2)
        die("usage: day24 file");
    else
        solve(read_file(argv[1]));
    return 0;
}
