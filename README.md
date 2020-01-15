# Enchant

Elixir bindings for the Enchant spellchecker

## Installation

The package can be installed by adding `enchant` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:enchant, "~> 0.1.0"}
  ]
end
```

The docs can be found at [https://hexdocs.pm/enchant](https://hexdocs.pm/enchant).

## What is Enchant?

Enchant is used to check the spelling of words and suggest corrections for words that are miss-spelled. It can use many popular spellchecking packages to perform this task, including ispell, aspell and MySpell. It is quite flexible at handling multiple dictionaries and multiple languages.
More information is available on [the Enchant website](http://www.abisource.com/enchant)

## Rust?

This package uses the excellent rustler package to bundle safely a NIF inside Elixir.
All the logix is handled by Rust since there was already a crate `enchant` with the bindings.

So we have the simple Elixir module `Enchant` that implements [Ruslter](https://github.com/rusterlium/rustler) contract to talk to the compiled rust code `enchant-elixir`.
The local crate pulls in the [enchant](https://crates.io/crates/enchant) crate that gives access to the binding exposed in [libenchant](https://github.com/AbiWord/enchant)

## API

Only `check/2` is exposed. All the dictionnary and broker from `libenchant` are not exposed because I do not need them right now.

## Usage

```
iex> Enchant.check("Enchanté, je vous présente ma librairie", "fr_CA")
{:ok,
 [
   %Enchant.Word{correct: true, suggestions: [], word: "Enchanté"},
   %Enchant.Word{correct: true, suggestions: [], word: "je"},
   %Enchant.Word{correct: true, suggestions: [], word: "vous"},
   %Enchant.Word{correct: true, suggestions: [], word: "présente"},
   %Enchant.Word{correct: true, suggestions: [], word: "ma"},
   %Enchant.Word{correct: true, suggestions: [], word: "librairie"}
 ]}
iex> Enchant.check("Ci sono sette giorni in una setimana.", "it")
{:ok,
 [
   ...
   %Enchant.Word{
     correct: false,
     suggestions: ["settimana", "estimata", ...],
     word: "setimana"
   }
 ]}
```
