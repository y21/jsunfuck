# JSUnfuck

> ⚠ This is a work in progress.

[JSFuck](http://jsfuck.com) is an esoteric programming style for JavaScript that makes it possible to write any valid JavaScript code with just six characters: `()+[]!`.

JSUnfuck reverses/simplifies the obfuscated output, so you can read it again.
```sh
$ cat input.js
[+!+[]] + [!+[] + !+[]] + [!+[] + !+[] + !+[]]

$ cat input.js | jsunfuck read 
"123";
```

There are existing tools for this already, many of which approach this by using dictionaries to search for patterns.
This works for output directly from one particular JSFuck code generator, however as soon as the dictionary is different, it no longer works and it requires you to know the dictionary that was used.
For example, `[+[]]+[]`, `[+[]]+[[]]` and `[]+[+![]]` all produce the string `"0"`. This program understands all of those.
This project tries to do this by deducing the syntax tree, without relying on a dictionary.
