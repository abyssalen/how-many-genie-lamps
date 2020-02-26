# How Many Genie Lamps (GMGL)


GMGL is a simple command-line application that calculates how many of an experience item such as
the [Genie Lamp][lamp] in [Old School RuneScape][osrs] that is required
to reach a certain level or experience.

Wrote this as my first [Rust][rust] application.

## Usage

```
 $ how-many-genie-lamps --help

 How Many Genie Lamps? 0.1
 Calculates how many genie lamps are required to meet a certain level or experience
 
 USAGE:
     how-many-genie-lamps.exe [OPTIONS]
 
 FLAGS:
     -h, --help       Prints help information
     -V, --version    Prints version information
 
 OPTIONS:
         --starting-level <LEVEL>       Sets the starting level
         --starting-xp <STARTING-XP>    Sets the starting xp
         --target-level <LEVEL>         Sets the target level
         --target-xp <target-XP>        Sets the target xp
```

#### Example

If you want to calculate from level 1 to level 99, run the following command:

`$ how-many-genie-lamps --starting-level 1 --target-level 99`

or 

`$ how-many-genie-lamps --starting-xp 0 --target-xp 13034431`

Which will both output:

```
Starting = [level: 1, xp: 0]
Target = [level: 99, xp: 13,034,431]
You need 15,057 lamps to reach the target.
```

Note that you can mix and match xp and levels for starting and target skills. 
So you can do this:

`$ how-many-genie-lamps --starting-xp 0 --target-level 99`

or

`$ how-many-genie-lamps --starting-level 1 --target-xp 13034431`

Which will both output the same result:

```
Starting = [level: 1, xp: 0]
Target = [level: 99, xp: 13,034,431]
You need 15,057 lamps to reach the target.
```

## TODO

* Only [Genie Lamps][lamp] are supported

I would like to include other experience items 
such as the [Book of Knowledge][book_knowledge].

* Possibly a GUI on the web or on desktop

## Open Source Projects Used

* [clap](https://github.com/clap-rs/clap)


[lamp]: https://oldschool.runescape.wiki/w/Lamp
[book_knowledge]: https://oldschool.runescape.wiki/w/Book_of_knowledge
[osrs]: https://oldschool.runescape.com/
[rust]: https://www.rust-lang.org/