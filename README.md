# YUCK

## A simple sandbox for programming language design, written in Rust.

This is intended to be an easy way to validate programming language designs,
without having to write a complicated and optimal compiler. This is not intended
to generate performant programming languages.

Provide the grammar in the following format, for instance:

```
file:
statement_list

statement_list:
statement
statement statement_list

statement:
assignment
func_call

assignment:
expression ASSIGN expression

expression:
expression MULTIPLY expression
expression ADD expression
IDENTIFIER
literal

literal:
INTEGER
STRING
```

The actual names don't matter. The parser will try to match against the first
rule. The order of the rules in the file match their priority.

Provide the token patterns in the following format, for instance:

```
INTEGER	([0-9])+
ASSIGN	=
MULTIPLY	*
ADD	+
STRING	"([^"]*)"
IDENTIFIER	[_a-zA-Z]\w*
```

Note the tab character separating the name from the regex. The first capture
group will be available to the lexer when constructing the parse tree.