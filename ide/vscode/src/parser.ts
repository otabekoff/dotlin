export type Expression =
    | { type: "NumberLiteral"; value: number }
    | { type: "Identifier"; name: string }
    | { type: "Unary"; op: string; expr: Expression }
    | { type: "Binary"; op: string; left: Expression; right: Expression }
    | { type: "Call"; callee: Expression; args: Expression[] };

export type Statement =
    | { type: "ExpressionStatement"; expression: Expression }
    | { type: "ReturnStatement"; argument?: Expression }
    | {
        type: "FunctionDeclaration";
        name: string;
        params: string[];
        body: Statement[];
    }
    | {
        type: "VariableDeclaration";
        kind: "val" | "var";
        name: string;
        init?: Expression;
    };

export type Node = { type: "Program"; body: Statement[] } | Expression;

type Token = { type: string; value?: string };

class Lexer {
    input: string;
    pos = 0;
    constructor(input: string) {
        this.input = input;
    }
    peek(): string {
        return this.input[this.pos] ?? "\0";
    }
    next(): string {
        return this.input[this.pos++] ?? "\0";
    }
    isAlpha(ch: string) {
        return /[a-zA-Z_]/.test(ch);
    }
    isDigit(ch: string) {
        return /[0-9]/.test(ch);
    }
    skipWhitespace() {
        while (/[\s]/.test(this.peek())) this.next();
    }
    tokenize(): Token[] {
        const out: Token[] = [];
        while (this.pos < this.input.length) {
            this.skipWhitespace();
            const ch = this.peek();
            if (ch === "\0") break;
            if (
                this.isDigit(ch) ||
                (ch === "." && this.isDigit(this.input[this.pos + 1] ?? ""))
            ) {
                let num = "";
                while (this.isDigit(this.peek()) || this.peek() === ".") {
                    num += this.next();
                }
                out.push({ type: "Number", value: num });
                continue;
            }
            if (this.isAlpha(ch)) {
                let id = "";
                while (this.isAlpha(this.peek()) || this.isDigit(this.peek())) {
                    id += this.next();
                }
                out.push({ type: "Identifier", value: id });
                continue;
            }
            // operators and punctuation
            const one = this.next();
            if ("+-*/^(){};,=,".includes(one)) {
                out.push({ type: one });
                continue;
            }
            throw new Error("Unexpected char: " + one);
        }
        out.push({ type: "EOF" });
        return out;
    }
}

export class Parser {
    tokens: Token[];
    pos = 0;
    constructor(input: string) {
        const lexer = new Lexer(input);
        this.tokens = lexer.tokenize();
    }
    peek(): Token {
        return this.tokens[this.pos];
    }
    next(): Token {
        return this.tokens[this.pos++];
    }

    // Precedences: higher number = tighter binding
    static PRECEDENCE: Record<string, number> = {
        "^": 4,
        "*": 3,
        "/": 3,
        "+": 2,
        "-": 2,
    };

    parse(): Node {
        return this.parseProgram();
    }

    parseProgram(): Node {
        const body: Statement[] = [];
        while (this.peek().type !== "EOF") {
            body.push(this.parseStatement());
        }
        return { type: "Program", body };
    }

    parseStatement(): Statement {
        const tok = this.peek();
        if (tok.type === "Identifier" && tok.value === "fun") {
            // function declaration
            this.next(); // consume 'fun'
            const nameTok = this.peek();
            if (nameTok.type !== "Identifier") {
                throw new Error("Expected function name");
            }
            const name = nameTok.value ?? "";
            this.next();
            if (this.peek().type !== "(") {
                throw new Error("Expected ( after function name");
            }
            this.next();
            const params: string[] = [];
            while (this.peek().type !== ")") {
                const p = this.peek();
                if (p.type !== "Identifier") {
                    throw new Error("Expected parameter name");
                }
                params.push(p.value ?? "");
                this.next();
                if (this.peek().type === ",") this.next();
            }
            this.next(); // )
            if (this.peek().type !== "{") throw new Error("Expected {");
            this.next();
            const body: Statement[] = [];
            while (this.peek().type !== "}") {
                body.push(this.parseStatement());
            }
            this.next(); // }
            return { type: "FunctionDeclaration", name, params, body };
        }
        if (tok.type === "Identifier" && tok.value === "return") {
            this.next();
            if (this.peek().type === ";") {
                this.next();
                return { type: "ReturnStatement" };
            }
            const arg = this.parseExpression(0);
            if (this.peek().type === ";") this.next();
            return { type: "ReturnStatement", argument: arg };
        }
        if (
            tok.type === "Identifier" &&
            (tok.value === "val" || tok.value === "var")
        ) {
            const kind = tok.value as "val" | "var";
            this.next();
            const nameTok = this.peek();
            if (nameTok.type !== "Identifier") {
                throw new Error("Expected variable name");
            }
            const name = nameTok.value ?? "";
            this.next();
            let init: Expression | undefined = undefined;
            if (this.peek().type === "=") {
                this.next();
                init = this.parseExpression(0);
            }
            if (this.peek().type === ";") this.next();
            return { type: "VariableDeclaration", kind, name, init };
        }
        // expression statement
        const expr = this.parseExpression(0);
        if (this.peek().type === ";") this.next();
        return { type: "ExpressionStatement", expression: expr };
    }

    parseExpression(minPrec: number): Expression {
        let left = this.parsePrefix();

        while (true) {
            const tok = this.peek();
            const op = tok.type;
            if (!(op in Parser.PRECEDENCE)) {
                // function call
                if (tok.type === "(") {
                    // call with no callee? treat as grouping (should be handled in prefix)
                }
                break;
            }
            const prec = Parser.PRECEDENCE[op];
            // right-assoc for ^
            const nextMin = op === "^" ? prec : prec + 1;
            if (prec < minPrec) break;
            this.next(); // consume op
            const right = this.parseExpression(nextMin);
            left = { type: "Binary", op, left, right };
        }
        return left;
    }

    parsePrefix(): Expression {
        const tok = this.peek();
        if (tok.type === "Number") {
            this.next();
            return { type: "NumberLiteral", value: Number(tok.value) };
        }
        if (tok.type === "Identifier") {
            this.next();
            let node: Expression = {
                type: "Identifier",
                name: tok.value ?? "",
            };
            // possible call
            if (this.peek().type === "(") {
                this.next();
                const args: Expression[] = [];
                while (this.peek().type !== ")") {
                    args.push(this.parseExpression(0));
                    if (this.peek().type === ",") this.next();
                }
                this.next(); // )
                node = { type: "Call", callee: node, args };
            }
            return node;
        }
        if (tok.type === "(") {
            this.next();
            const expr = this.parseExpression(0);
            if (this.peek().type !== ")") throw new Error("Expected )");
            this.next();
            return expr;
        }
        if (tok.type === "+" || tok.type === "-") {
            this.next();
            const expr = this.parseExpression(4); // unary binds tight
            return { type: "Unary", op: tok.type, expr };
        }
        throw new Error("Unexpected token in prefix: " + tok.type);
    }
}

export function evaluate(
    node: Expression,
    env: Record<string, number | Function> = {},
): number {
    switch (node.type) {
        case "NumberLiteral":
            return node.value;
        case "Identifier":
            if (!(node.name in env)) {
                throw new Error("Unknown identifier: " + node.name);
            }
            const v = env[node.name];
            if (typeof v === "number") return v;
            throw new Error("Identifier is not a number: " + node.name);
        case "Unary": {
            const v2 = evaluate(node.expr, env);
            if (node.op === "+") return +v2;
            if (node.op === "-") return -v2;
            throw new Error("Unknown unary op: " + node.op);
        }
        case "Binary": {
            const L = evaluate(node.left, env);
            const R = evaluate(node.right, env);
            switch (node.op) {
                case "+":
                    return L + R;
                case "-":
                    return L - R;
                case "*":
                    return L * R;
                case "/":
                    return L / R;
                case "^":
                    return Math.pow(L, R);
            }
            throw new Error("Unknown binary op: " + node.op);
        }
        case "Call": {
            const callee = node.callee;
            if (callee.type !== "Identifier") {
                throw new Error("Only simple identifier calls supported");
            }
            const fn = env[callee.name];
            if (typeof fn !== "function") {
                throw new Error("Unknown function: " + callee.name);
            }
            const args = node.args.map((a) => evaluate(a, env));
            return fn(...args);
        }
    }
}

export function parseExpression(input: string): Expression {
    const p = new Parser(input);
    const expr = p.parseExpression(0);
    const rem = p.peek();
    if (rem.type !== "EOF" && rem.type !== ";" && rem.type !== ")") {
        throw new Error("Unexpected token after expression");
    }
    return expr;
}

export function parseProgram(input: string): Node {
    const p = new Parser(input);
    return p.parse();
}
