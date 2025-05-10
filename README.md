# Fpas


fpas is command line software helps you create password from some text 


## Installation

 ```bash
 cargo install fpas
 ```

## Usage

create password with text

```bash
fpas your_text
```

and loop feature

```bash
fpas -l 5 your_text
```

create password from binary file

```bash
fpas -f ./file
```

using `--chain` with `-l` (or `--loop`)
to generate long password

```bash
fpas --chain -l 10 your_text
```

---


#### Use `-i` or `--input` to cover your key

let consider this command `7z a zipfile folder -p$(fpas 'your_key')`: after you'd enter the command, your shell will keep all commands and options in history (use `history` command to print history) which increases security risks.

To solve this problem simply use `-i`.

```bash
fpas -i
> your_key
&9@a7df@5977b5b009b3@06#054c7f7@3363b##0c...
```

after you have seen `>` just enter the commands and options which you prefer. Then just press `enter` on your keyboard.

## Use cases

You might want to create a compressed file with a password. To ensure the strongest security, you need to use a very long password that includes a variety of characters.

So, I created this software to solve this problem and create strong passwords with a simple method. However, it actually just transferred the weakness to other points.

There is example cases with command line.

```bash
7z a file.7z file -p$(fpas helloG)
```

```bash
zip file.zip file -P $(fpas hello)