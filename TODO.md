

---
You're right! VS Code's markdown preview doesn't automatically recognize `.lin` or `dotlin` syntax highlighting because it's a custom language. Here are the solutions:

```dotlin
fun main() {
    println("Hello!")
}
```
---
otabek@mohirlab:~/Projects/langs/dotlin$  cd /home/otabek/Projects/langs/dotlin/ide/vscode && npm ci && npm run compile
npm warn deprecated whatwg-encoding@3.1.1: Use @exodus/bytes instead for a more spec-conformant and faster implementation

added 338 packages, and audited 339 packages in 8s

88 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> dotlin-language@0.1.0 compile
> tsc -p ./

otabek@mohirlab:~/Projects/langs/dotlin/ide/vscode$  npm run package

> dotlin-language@0.1.0 package
> vsce package

Executing prepublish script 'npm run vscode:prepublish'...

> dotlin-language@0.1.0 vscode:prepublish
> npm run compile


> dotlin-language@0.1.0 compile
> tsc -p ./

OK, : Use @exodus/bytes.
