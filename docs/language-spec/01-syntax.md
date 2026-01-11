# Comments

Just like most modern languages, Dotlin supports single-line (or end-of-line) and multi-line (block) comments:

```lin
// This is an end-of-line comment

/* This is a block comment
	on multiple lines. */
```

Block comments in Dotlin can be nested:

```lin
/* The comment starts here
/* contains a nested comment */
and ends here. */
```

Block comments may nest to arbitrary depth; lexers and parsers should account for nested block comment delimiters when scanning tokens.

