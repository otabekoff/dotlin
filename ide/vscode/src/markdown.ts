export default function dotlinMarkdownPlugin(md: any) {
    // debug: indicate plugin was loaded
    try {
        console.debug("dotlin: markdown plugin loaded");
    } catch {}
    // Replace info string of fenced code blocks `lin`, `dotlin` or `kotlin` with `dotlin`
    md.core.ruler.push("dotlin-fence-lang", function (state: any) {
        for (const token of state.tokens) {
            if (token.type === "fence") {
                const info = (token.info || "").trim().split(/\s+/)[0];
                if (info === "lin" || info === "dotlin" || info === "kotlin") {
                    // debug: log mapping
                    try {
                        console.debug(
                            `dotlin: mapping fenced lang '${info}' -> 'dotlin'`,
                        );
                    } catch {}
                    // set to dotlin so markdown preview uses the Dotlin TextMate grammar
                    token.info = "dotlin" +
                        (token.info ? token.info.slice(info.length) : "");
                }
            }
        }
    });
}
