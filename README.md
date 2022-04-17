# JSUnfuck

> ⚠ This is a work in progress.

[JSFuck](http://jsfuck.com) is an esoteric programming style for JavaScript that makes it possible to write any valid JavaScript code with just six characters: `()+[]!`.

JSUnfuck reverses/simplifies the obfuscated output, so you can read it again.
```sh
$ cat input.js
[+!+[]] + [!+[] + !+[]] + [!+[] + !+[] + !+[]]

$ cat input.js | jsunfuck read 
123;
```
