# How Many Genie Lamps (HMGL)

HMGL is a simple command-line application that calculates how many [genie lamps][lamp] or [books of knowledge][book_knowledge]
are required to reach a certain level or experience.

Wrote this as my first [Rust](https://www.rust-lang.org/) application.

## Usage

```
$ how-many-genie-lamps --help

How Many Genie Lamps? 0.1
Calculates how many genie lamps and books of knowledge are required to meet a certain level or experience
 
USAGE:
    how-many-genie-lamps.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --start-lvl <LEVEL>         Sets the starting level
        --start-xp <STARTING-XP>    Sets the starting xp
        --end-lvl <LEVEL>           Sets the target level
        --end-xp <TARGET-XP>        Sets the target xp
```

#### Example

If you want to calculate from level 1 to level 99, run the following command:

`$ how-many-genie-lamps --start-lvl 1 --end-lvl 99`

or 

`$ how-many-genie-lamps --start-xp 0 --end-xp 13034431`

Which will both output:

```
Starting = [level: 1, xp: 0]
Target = [level: 99, xp: 13,034,431]
You need 15,057 lamps or 10,039 books to reach the target.
```

Note that you can mix and match xp and levels for starting and target skills. 
So you can do this:

`$ how-many-genie-lamps --start-xp 0 --end-lvl 99`

or

`$ how-many-genie-lamps --start-lvl 1 --end-xp 13034431`

Which will both output the same result:

```
Starting = [level: 1, xp: 0]
Target = [level: 99, xp: 13,034,431]
You need 15,057 lamps or 10,039 books to reach the target.
```

## TODO

* Possibly a GUI on the web or on desktop

* Allow users to enter numbers that are formatted with a comma such as `200,000,000`

## Open Source Projects Used

* [clap](https://github.com/clap-rs/clap)
* [num-format](https://github.com/bcmyers/num-format)


[lamp]: https://oldschool.runescape.wiki/w/Lamp
[book_knowledge]: https://oldschool.runescape.wiki/w/Book_of_knowledge
[osrs]: https://oldschool.runescape.com/