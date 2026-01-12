export default function createShikiPlugin(highlighter: any) {
    return function shikiPlugin(md: any) {
        const origFence = md.renderer.rules.fence;

        md.renderer.rules.fence = function (
            tokens: any[],
            idx: number,
            options: any,
            env: any,
            slf: any,
        ) {
            const token = tokens[idx];
            const info = (token.info || "").trim();
            const first = info.split(/\s+/)[0] || "";
            const lang = /^(lin|dotlin)$/i.test(first)
                ? "kotlin"
                : first || "text";
            try {
                // Shiki returns a full HTML string (pre/code) — return it directly
                return highlighter.codeToHtml(token.content, { lang });
            } catch (e) {
                // fallback to original renderer
                if (origFence) {
                    return origFence.call(this, tokens, idx, options, env, slf);
                }
                return slf.renderToken(tokens, idx, options);
            }
        };
    };
}
