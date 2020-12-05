# logic-minimizer
to (not) die from ac 

# Requirements
* [Rust](https://www.rust-lang.org/tools/install)

# Usage
1. Install rust via the link above
2. Clone this repo with `git clone https://github.com/gidra5/logic-minimizer.git`
3. Create your txt file with your table and labels according to the rules below and put in directory or "example" folder
4. Run `cargo run <your txt file name>`
  
# Formatting  
Txt file for your table should follow some rules, so that program didn't fail or generate incorrect results.

1. First line describes labels separated by ` ` and one `~` for separating input and output. 
There should be no spaces between `~` and adjacent labels for now. 
2. 2 lines below labels starts truth-table of your logic functions. 
Inputs and outputs are separated by `~`. 
The only valid characters are `0` `1` and `-`.

In future we may improve formating and soften some of these rules [#1](https://github.com/gidra5/logic-minimizer/issues/1) or replace them with descriptive messages from the program [#2](https://github.com/gidra5/logic-minimizer/issues/2).
