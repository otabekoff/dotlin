export type Expression =
    | { type: "NumberLiteral"; value: number }
    | { type: "StringLiteral"; value: string }
    | { type: "Identifier"; name: string }
    | { type: "Index"; object: Expression; index: Expression }
    | { type: "Member"; object: Expression; property: string }
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
        typeName?: string;
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
            // skip comments: // line comments and /* block comments */
            if (this.peek() === "/" && this.input[this.pos + 1] === "/") {
                // line comment
                this.next();
                this.next();
                while (this.pos < this.input.length && this.peek() !== "\n") {
                    this.next();
                }
                continue;
            }
            if (this.peek() === "/" && this.input[this.pos + 1] === "*") {
                // block comment
                this.next();
                this.next();
                while (this.pos < this.input.length) {
                    if (
                        this.peek() === "*" && this.input[this.pos + 1] === "/"
                    ) {
                        this.next();
                        this.next();
                        break;
                    }
                    this.next();
                }
                continue;
            }
            const ch = this.peek();
            if (ch === "\0") break;
            // string literals
            if (ch === '"' || ch === "'") {
                const quote = this.next();
                let str = "";
                while (this.pos < this.input.length) {
                    const c = this.next();
                    if (c === "\\") {
                        // escape
                        const nxt = this.next();
                        str += nxt === undefined ? "\\" : "\\" + nxt;
                        continue;
                    }
                    if (c === quote) break;
                    str += c;
                }
                out.push({ type: "String", value: str });
                continue;
            }
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
            // operators and punctuation: accept common punctuation used in Dotlin
            const one = this.next();
            if (/[+\-*/^(){}\[\];,=:.<>!&|?:$]/.test(one)) {
                out.push({ type: one });
                continue;
            }
            // allow other single-char punctuation (fallback)
            if (/\p{P}/u.test(one)) {
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
        // handle package/import as top-level declarations (skip their content)
        if (
            tok.type === "Identifier" &&
            (tok.value === "import" || tok.value === "package")
        ) {
            this.next();
            // consume until semicolon or end of input
            while (this.peek().type !== ";" && this.peek().type !== "EOF") {
                this.next();
            }
            if (this.peek().type === ";") this.next();
            // represent as an empty expression statement so callers won't error
            return {
                type: "ExpressionStatement",
                expression: { type: "Identifier", name: tok.value ?? "" },
            };
        }
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
                // optional type annotation: skip tokens after ':' until comma or ')'
                if (this.peek().type === ":") {
                    this.next(); // consume ':'
                    // skip a simple type or generic type tokens
                    let depth = 0;
                    while (
                        this.peek().type !== ")" &&
                        this.peek().type !== ","
                    ) {
                        const t = this.peek().type;
                        if (t === "<") {
                            depth++;
                        } else if (t === ">") {
                            if (depth > 0) depth--;
                        }
                        this.next();
                        if (
                            depth <= 0 &&
                            (this.peek().type === "," ||
                                this.peek().type === ")")
                        ) break;
                    }
                }
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
            let typeName: string | undefined = undefined;
            let init: Expression | undefined = undefined;

            // optional type annotation
            if (this.peek().type === ":") {
                this.next(); // consume ':'
                // capture a simple identifier type name (e.g., Int, String)
                if (this.peek().type === "Identifier") {
                    typeName = this.peek().value ?? undefined;
                    this.next();
                } else {
                    // if unexpected, surface a clear error for editor
                    throw new Error("Expected type name after ':'");
                }
            }

            // optional initializer
            if (this.peek().type === "=") {
                this.next();
                init = this.parseExpression(0);
            }

            if (!init && !typeName) {
                // Enforce explicit-type requirement for uninitialized declarations.
                throw new Error(
                    "Uninitialized variable requires explicit type",
                );
            }

            if (this.peek().type === ";") this.next();
            return { type: "VariableDeclaration", kind, name, typeName, init };
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
        if (tok.type === "String") {
            this.next();
            return { type: "StringLiteral", value: tok.value ?? "" };
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
            // handle postfix: indexing `obj[index]` and member access `obj.prop`
            while (true) {
                if (this.peek().type === "[") {
                    this.next(); // [
                    const idx = this.parseExpression(0);
                    if (this.peek().type !== "]") throw new Error("Expected ]");
                    this.next();
                    node = {
                        type: "Index",
                        object: node,
                        index: idx,
                    } as Expression;
                    continue;
                }
                if (this.peek().type === ".") {
                    this.next();
                    const prop = this.peek();
                    if (prop.type !== "Identifier") {
                        throw new Error("Expected property name after .");
                    }
                    this.next();
                    node = {
                        type: "Member",
                        object: node,
                        property: prop.value ?? "",
                    } as Expression;
                    continue;
                }
                break;
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
        case "StringLiteral":
            throw new Error("String literal in numeric expression");
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
        case "Index": {
            throw new Error(
                "Indexing expressions are not supported in numeric evaluation",
            );
        }
        case "Member": {
            throw new Error(
                "Member access is not supported in numeric evaluation",
            );
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

// Infer simple type from later usage in the source text.
// Returns one of: "Int", "String", "Boolean" or null if unknown.
export function inferTypeFromUsage(text: string, name: string): string | null {
    // Detect usage patterns to infer a simple type: Int, Double, String, Boolean, Array<...>
    const strRe = new RegExp(
        "\\b" + name + "\\b\\s*(?:=|\\+=)\\s*(\\\"([^\\\\\"]*)\\\"|'([^']*)')",
        "g",
    );
    const boolRe = new RegExp(
        "\\b" + name + "\\b\\s*(?:=|\\+=)\\s*\\b(true|false)\\b",
        "g",
    );
    const floatRe = new RegExp(
        "\\b" + name + "\\b\\s*(?:=|\\+=)\\s*([-+]?[0-9]*\\.[0-9]+)",
        "g",
    );
    const intRe = new RegExp(
        "\\b" + name + "\\b\\s*(?:=|\\+=)\\s*([-+]?[0-9]+)\\b",
        "g",
    );
    const arrayRe = new RegExp(
        "\\b" + name + "\\b\\s*(?:=|\\+=)\\s*\\[([^\\]]*)\\]",
        "g",
    );

    let m: RegExpExecArray | null;
    if ((m = strRe.exec(text)) !== null) return "String";
    if ((m = boolRe.exec(text)) !== null) return "Boolean";
    if ((m = arrayRe.exec(text)) !== null) {
        const contents = m[1] || "";
        const first = contents.split(/,/)[0]?.trim() ?? "";
        if (/^\".*\"$/.test(first) || /^'.*'$/.test(first)) {
            return "Array<String>";
        }
        if (/[-+]?[0-9]*\.[0-9]+/.test(first)) return "Array<Double>";
        if (/[-+]?[0-9]+$/.test(first)) return "Array<Int>";
        return "Array<Any>";
    }
    if ((m = floatRe.exec(text)) !== null) return "Double";
    if ((m = intRe.exec(text)) !== null) return "Int";
    return null;
}
