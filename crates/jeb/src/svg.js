document.documentElement.dataset.interactive = "";
for (const def of document.querySelectorAll("defs[data-interactive]")) {
    for (const child of def.children) {
        def.before(child);
    }
    def.remove();
}
sv;
