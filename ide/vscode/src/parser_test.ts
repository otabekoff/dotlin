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

// Additional parser tests: ensure parseProgram surfaces explicit-type error
import { inferTypeFromUsage, parseProgram } from "./parser.js";

try {
    let thrown = false;
    try {
        parseProgram("val x;");
    } catch (e: any) {
        thrown = true;
        if (
            String(e.message || e) !==
                "Uninitialized variable requires explicit type"
        ) {
            console.error(
                "Unexpected parser error message:",
                String(e.message || e),
            );
            process.exit(1);
        }
    }
    if (!thrown) {
        console.error(
            "Expected parseProgram to throw for uninitialized var without type",
        );
        process.exit(1);
    }

    // This should parse successfully
    const prog = parseProgram("val x: Int;\nvar y = 5;");
    if (prog.type !== "Program") {
        console.error("Expected Program node");
        process.exit(1);
    }
    console.log("Parser diagnostic tests passed");
    // Tests for inferTypeFromUsage
    const sample1 = `val x;\nx = 3;`;
    const t1 = inferTypeFromUsage(sample1, "x");
    if (t1 !== "Int") {
        console.error(
            "inferTypeFromUsage failed for numeric assignment, got:",
            t1,
        );
        process.exit(1);
    }
    const sample2 = `val s;\ns = "hello";`;
    const t2 = inferTypeFromUsage(sample2, "s");
    if (t2 !== "String") {
        console.error(
            "inferTypeFromUsage failed for string assignment, got:",
            t2,
        );
        process.exit(1);
    }
    const sample3 = `val b;\nb = true;`;
    const t3 = inferTypeFromUsage(sample3, "b");
    if (t3 !== "Boolean") {
        console.error(
            "inferTypeFromUsage failed for boolean assignment, got:",
            t3,
        );
        process.exit(1);
    }
    console.log("Type inference tests passed");
    // Additional inference cases: float and arrays
    const sample4 = `val f;\nf = 3.14;`;
    const t4 = inferTypeFromUsage(sample4, "f");
    if (t4 !== "Double") {
        console.error(
            "inferTypeFromUsage failed for float assignment, got:",
            t4,
        );
        process.exit(1);
    }
    const sample5 = `val arr;\narr = [1,2,3];`;
    const t5 = inferTypeFromUsage(sample5, "arr");
    if (t5 !== "Array<Int>") {
        console.error("inferTypeFromUsage failed for array<int>, got:", t5);
        process.exit(1);
    }
    const sample6 = `val arr2;\narr2 = [1.2,3.4];`;
    const t6 = inferTypeFromUsage(sample6, "arr2");
    if (t6 !== "Array<Double>") {
        console.error("inferTypeFromUsage failed for array<double>, got:", t6);
        process.exit(1);
    }
    console.log("Extended type inference tests passed");
    // Test extension quick-fix helper (use dynamic import for ESM)
    await (async () => {
        const ext = await import("./quickfix_helper.js");
        const q1 = ext.computeQuickFixForUninitialized("val x;\nx = 3;");
        if (!q1 || q1.suggested !== ": Int") {
            console.error("Quick-fix helper failed for numeric case:", q1);
            process.exit(1);
        }
        const q2 = ext.computeQuickFixForUninitialized('val s;\ns = "hi";');
        if (!q2 || q2.suggested !== ": String") {
            console.error("Quick-fix helper failed for string case:", q2);
            process.exit(1);
        }
        const q3 = ext.computeQuickFixForUninitialized("val b;\nb = true;");
        if (!q3 || q3.suggested !== ": Boolean") {
            console.error("Quick-fix helper failed for boolean case:", q3);
            process.exit(1);
        }
        console.log("Extension quick-fix helper tests passed");
    })();
} catch (e) {
    console.error("Parser tests failed:", e);
    process.exit(1);
}
