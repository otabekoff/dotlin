import * as vscode from "vscode";
import { evaluate, parseExpression, parseProgram } from "./parser.js";
import computeQuickFixForUninitialized from "./quickfix_helper.js";
import createShikiPlugin from "./markdown-shiki.js";

let shikiPlugin: any = null;

export function activate(context: vscode.ExtensionContext) {
    const selector: vscode.DocumentSelector = {
        language: "dotlin",
        scheme: "file",
    };

    const keywords = ["fun", "val", "var", "class", "if", "else", "when"];

    const completionProvider = vscode.languages.registerCompletionItemProvider(
        selector,
        {
            provideCompletionItems() {
                return keywords.map((k) =>
                    new vscode.CompletionItem(
                        k,
                        vscode.CompletionItemKind.Keyword,
                    )
                );
            },
        },
    );

    const hoverProvider = vscode.languages.registerHoverProvider(selector, {
        provideHover(document, position) {
            const range = document.getWordRangeAtPosition(position);
            if (!range) return undefined;
            const word = document.getText(range);
            switch (word) {
                case "fun":
                    return new vscode.Hover("`fun` — function declaration");
                case "val":
                    return new vscode.Hover("`val` — immutable variable");
                case "var":
                    return new vscode.Hover("`var` — mutable variable");
                default:
                    return undefined;
            }
        },
    });

    const diagnostics = vscode.languages.createDiagnosticCollection("dotlin");

    async function validateTextDocument(doc: vscode.TextDocument) {
        if (doc.languageId !== "dotlin") return;
        const text = doc.getText();
        const diags: vscode.Diagnostic[] = [];
        try {
            const program = parseProgram(text);
            // simple semantic: return at top-level is invalid
            if (program.type === "Program") {
                for (const stmt of program.body) {
                    if (stmt.type === "ReturnStatement") {
                        const idx = text.indexOf("return");
                        const pos = doc.positionAt(idx >= 0 ? idx : 0);
                        const range = new vscode.Range(
                            pos,
                            pos.translate(0, 6),
                        );
                        diags.push(
                            new vscode.Diagnostic(
                                range,
                                "`return` at top-level: returns must be inside functions",
                                vscode.DiagnosticSeverity.Error,
                            ),
                        );
                        break;
                    }
                }
            }
        } catch (e: any) {
            const msg = String(e.message || e);
            // try to locate the offending variable declaration for better diagnostics
            if (msg === "Uninitialized variable requires explicit type") {
                const r = findUninitializedVarRange(doc, text);
                const range = r ??
                    new vscode.Range(
                        new vscode.Position(0, 0),
                        new vscode.Position(0, 0),
                    );
                diags.push(
                    new vscode.Diagnostic(
                        range,
                        msg,
                        vscode.DiagnosticSeverity.Error,
                    ),
                );
            } else {
                const pos = new vscode.Position(0, 0);
                const range = new vscode.Range(pos, pos);
                diags.push(
                    new vscode.Diagnostic(
                        range,
                        msg,
                        vscode.DiagnosticSeverity.Error,
                    ),
                );
            }
        }
        diagnostics.set(doc.uri, diags);
    }

    function findUninitializedVarRange(
        doc: vscode.TextDocument,
        text: string,
    ): vscode.Range | null {
        const q = computeQuickFixForUninitialized(text);
        if (!q) return null;
        const nameStart = doc.positionAt(q.start);
        const nameEnd = doc.positionAt(q.end);
        return new vscode.Range(nameStart, nameEnd);
    }

    // Code action provider: quick-fix to insert ': Type' for uninitialized vars
    const codeActionProvider = vscode.languages.registerCodeActionsProvider(
        selector,
        {
            provideCodeActions(document, range, context) {
                const actions: vscode.CodeAction[] = [];
                for (const diag of context.diagnostics) {
                    if (
                        diag.message ===
                            "Uninitialized variable requires explicit type"
                    ) {
                        const docText = document.getText();
                        const q = computeQuickFixForUninitialized(docText);
                        if (!q) continue;
                        const nameStart = document.positionAt(q.start);
                        const nameEnd = document.positionAt(q.end);
                        const action = new vscode.CodeAction(
                            `Insert '${q.suggested}'`,
                            vscode.CodeActionKind.QuickFix,
                        );
                        const edit = new vscode.WorkspaceEdit();
                        edit.insert(document.uri, nameEnd, q.suggested);
                        action.edit = edit;
                        action.diagnostics = [diag];
                        actions.push(action);
                    }
                }
                return actions;
            },
        },
        { providedCodeActionKinds: [vscode.CodeActionKind.QuickFix] },
    );

    // validate open documents
    if (vscode.window.activeTextEditor) {
        validateTextDocument(vscode.window.activeTextEditor.document);
    }

    context.subscriptions.push(
        completionProvider,
        hoverProvider,
        diagnostics,
        codeActionProvider,
        vscode.workspace.onDidOpenTextDocument(validateTextDocument),
        vscode.workspace.onDidChangeTextDocument((e) =>
            validateTextDocument(e.document)
        ),
    );

    const evaluateCmd = vscode.commands.registerCommand(
        "dotlin.evaluate",
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) return;
            const sel = editor.selection;
            const text = sel.isEmpty
                ? editor.document.lineAt(sel.active.line).text
                : editor.document.getText(sel);
            try {
                const expr = parseExpression(text);
                const result = evaluate(expr, {
                    pow: Math.pow,
                    sin: Math.sin,
                    cos: Math.cos,
                });
                vscode.window.showInformationMessage(String(result));
            } catch (e: any) {
                vscode.window.showErrorMessage(String(e.message || e));
            }
        },
    );

    context.subscriptions.push(evaluateCmd);
    const restartCmd = vscode.commands.registerCommand(
        "dotlin.restart",
        async () => {
            diagnostics.clear();
            // re-validate all open Dotlin documents
            for (const doc of vscode.workspace.textDocuments) {
                try {
                    await validateTextDocument(doc);
                } catch (e) {
                    // ignore
                }
            }
            vscode.window.showInformationMessage(
                "Dotlin: diagnostics refreshed",
            );
        },
    );
    context.subscriptions.push(restartCmd);
    // Initialize Shiki asynchronously for Markdown preview highlighting.
    (async () => {
        try {
            const shikiModule: any = await import("shiki");
            const highlighter = shikiModule.createHighlighter
                ? await shikiModule.createHighlighter({ themes: ["nord"] })
                : await shikiModule.getHighlighter({ theme: "nord" });
            shikiPlugin = createShikiPlugin(highlighter);
            // no-op log
            // console.debug('dotlin: shiki highlighter ready');
        } catch (e) {
            // ignore — fallback will be used
            // console.warn('dotlin: shiki initialization failed', e);
        }
    })();
    // Do not return an extendMarkdownIt from activate; we export it at top-level.
}

export function deactivate() {}

// Helper for tests: compute quick-fix suggestion from plain text.
// (moved to quickfix_helper.ts)

// Provide a markdown-it plugin to the VS Code Markdown extension so preview
// highlights ```dotlin / ```lin blocks as Kotlin.
export function extendMarkdownIt(md: any) {
    if (shikiPlugin) {
        return md.use(shikiPlugin);
    }
    md.core.ruler.push("dotlin-fence-lang", (state: any) => {
        for (const token of state.tokens) {
            if (token.type === "fence" && token.info) {
                const info = (token.info || "").trim();
                const first = info.split(/\s+/)[0];
                if (/^(dotlin|lin)$/i.test(first)) {
                    // replace leading language id with kotlin, preserve attributes
                    token.info = info.replace(
                        /^(\s*)(dotlin|lin)/i,
                        (m: string, a: string) => (a || "") + "kotlin",
                    );
                }
            }
        }
    });
    return md;
}
