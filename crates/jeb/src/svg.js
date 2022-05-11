document.documentElement.classList.add("interactive");
for (const def of document.querySelectorAll("defs.interactive")) {
    for (const child of def.children) {
        def.before(child);
    }
    def.remove();
}
