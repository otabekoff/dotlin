// Preview script: register `dotlin` language for highlight.js and apply highlighting
(() => {
    try {
        try {
            console.debug("dotlin: preview highlight script loaded");
        } catch {}
        const hljs: any = (window as any).hljs;
        if (hljs) {
            try {
                if (!hljs.getLanguage || !hljs.getLanguage("dotlin")) {
                    // If Kotlin is available, reuse its definition for dotlin
                    const kotlinDef = hljs.getLanguage &&
                        hljs.getLanguage("kotlin");
                    if (kotlinDef) {
                        // register dotlin as an alias of kotlin
                        hljs.registerLanguage("dotlin", () => kotlinDef);
                    } else {
                        // Fallback: register a minimal Dotlin definition
                        hljs.registerLanguage("dotlin", () => ({
                            keywords: {
                                keyword:
                                    "fun val var class if else when return for while break continue true false null",
                            },
                            contains: [
                                hljs.QUOTE_STRING_MODE,
                                hljs.C_LINE_COMMENT_MODE,
                                hljs.C_BLOCK_COMMENT_MODE,
                                hljs.NUMBER_MODE,
                            ],
                        }));
                    }
                }
            } catch (e) {
                console.error("dotlin: language registration failed", e);
            }
        }

        function highlightAll() {
            const blocks = Array.from(document.querySelectorAll('pre code')) as HTMLElement[];
            try {
                console.debug(
                    `dotlin: highlightAll found ${blocks.length} blocks`,
                );
            } catch {}
            blocks.forEach((block) => {
                const el = block as HTMLElement;
                try {
                    const cls = (el.className || '').toLowerCase();
                    const infoLang = (cls.match(/language-([a-z0-9_-]+)/) || cls.match(/lang-([a-z0-9_-]+)/) || [])[1];
                    const wantsKotlin = infoLang === 'lin' || infoLang === 'dotlin' || infoLang === 'kotlin';
                    if (hljs && typeof hljs.highlightElement === 'function') {
                        if (wantsKotlin) {
                            // ensure class indicates kotlin so CSS applies
                            el.classList.add('language-kotlin');
                            try {
                                hljs.highlightElement(el as any);
                                return;
                            } catch {}
                        }
                        // fallback: try auto-detect with kotlin as hint
                        try {
                            if (typeof hljs.highlight === 'function') {
                                const res = hljs.highlight((infoLang === 'lin' || infoLang === 'dotlin') ? 'kotlin' : infoLang || 'kotlin', el.textContent || '');
                                el.innerHTML = res.value || el.innerHTML;
                                el.classList.add('language-kotlin');
                                return;
                            }
                        } catch {}
                        // final fallback: highlightElement without changes
                        try { hljs.highlightElement(el as any); } catch {}
                    }
                } catch (e) {
                    // silent
                }
            });
        }

        if (document.readyState === "loading") {
            document.addEventListener("DOMContentLoaded", highlightAll);
        } else {
            highlightAll();
        }

        // Re-run when the DOM changes (preview updates content dynamically)
        try {
            const mo = new MutationObserver(() => highlightAll());
            mo.observe(document.body, { childList: true, subtree: true });
        } catch {}
    } catch (e) {
        console.error("dotlin highlight preview error", e);
    }
})();
