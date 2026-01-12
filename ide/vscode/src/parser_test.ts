import { evaluate, parseExpression } from "./parser.js";

type Case = { expr: string; env?: Record<string, number>; expected: number };

const cases: Case[] = [
    { expr: "1+2*3", expected: 7 },
    { expr: "(1+2)*3", expected: 9 },
    { expr: "-3+5", expected: 2 },
    { expr: "2^3^2", expected: 512 },
    { expr: "a*2+3", env: { a: 4 }, expected: 11 },
    {
        expr: "3+4*2/(1-5)^2^3",
        expected: 3 + (4 * 2) / Math.pow(1 - 5, Math.pow(2, 3)),
    },
];

let failed = 0;
for (const c of cases) {
    try {
        const ast = parseExpression(c.expr);
        const got = evaluate(ast, c.env || {});
        const ok = Math.abs(got - c.expected) < 1e-9;
        console.log(
            c.expr,
            "=>",
            got,
            ok ? "OK" : `FAIL (expected ${c.expected})`,
        );
        if (!ok) failed++;
    } catch (e) {
        console.error("Error:", c.expr, e);
        failed++;
    }
}

if (failed > 0) {
    console.error(failed, "tests failed");
    process.exit(1);
} else {
    console.log("All tests passed");
}
