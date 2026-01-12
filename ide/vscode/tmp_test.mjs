import MarkdownIt from 'markdown-it';
import dotlinMarkdownPlugin from './out/markdown.js';

const md = new MarkdownIt();
md.use(dotlinMarkdownPlugin);

const input = '```lin\nfun main() { println("hi") }\n```\n';
console.log(md.render(input));
