import { inferTypeFromUsage } from "./parser.js";

export function computeQuickFixForUninitialized(
    text: string,
): { start: number; end: number; suggested: string } | null {
    const re = /(\b(?:val|var)\b)\s+([A-Za-z_][A-Za-z0-9_]*)/g;
    let m: RegExpExecArray | null;
    while ((m = re.exec(text)) !== null) {
        const startIdx = m.index;
        const name = m[2];
        const afterNameIdx = re.lastIndex;
        let i = afterNameIdx;
        while (i < text.length && /[\s]/.test(text[i])) i++;
        const next = text[i] ?? "";
        if (next === ":" || next === "=") continue;
        const namePos = text.indexOf(name, startIdx);
        const inferred = inferTypeFromUsage(text, name);
        const suggested = inferred ? `: ${inferred}` : ": Any";
        return { start: namePos, end: namePos + name.length, suggested };
    }
    return null;
}

export default computeQuickFixForUninitialized;
