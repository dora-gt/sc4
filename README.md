# sc4
sc4 = snake-case camel-case converter

## Install

```
$ cargo install --git https://github.com/dora-gt/sc4.git
```

## Usage
```
$ sc4 CamelCase
camel_case

$ sc4 snake_case
SnakeCase

$ sc4 -c kebab CamelCase
camel-case

# for Alfred
$ sc4 -a -c kebab CamelCase
{"items":[{"arg":"camel-case","subtitle":"converted from CamelCase","title":"Copy camel-case to Clipboard"}]}
```

for more, just type `sc4 -h`.

## Alfred
You will find Alfred workflow file in [Alfred](./Alfred) directory. Please import it.