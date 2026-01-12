import * as vscode from "vscode";
import { evaluate, parseExpression, parseProgram } from "./parser.js";

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
            const pos = new vscode.Position(0, 0);
            const range = new vscode.Range(pos, pos);
            diags.push(
                new vscode.Diagnostic(
                    range,
                    String(e.message || e),
                    vscode.DiagnosticSeverity.Error,
                ),
            );
        }
        diagnostics.set(doc.uri, diags);
    }

    // validate open documents
    if (vscode.window.activeTextEditor) {
        validateTextDocument(vscode.window.activeTextEditor.document);
    }

    context.subscriptions.push(
        completionProvider,
        hoverProvider,
        diagnostics,
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
}

export function deactivate() {}
