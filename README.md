# word-book

a simple glossary for japanese words, which supports add words and list/ search words by date.  

## add a new word to the glossary

Run command

```bash
word-book add こんにちは konnichiha "Good morning"
```

and you will see

```bash
word: こんにちは
romanji: konnichiha
meaning: Good morning
```

which means this word are already appended to the glossary.

## show and search words from the glossary

Run command

```bash
word-book list | column -t -s,
```

and you'll see words you add today. To print words you added before, run `list` command with option `-<num>` like the following.  

```bash
word-book list -- -5
```

This command will print words between today and five days ago.  
To search, you can use other command line tools like `grep`.
