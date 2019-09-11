# think-and-say

A small toy program to say with the meme "支離滅裂な思考・発言" in terminal.

## Installation

```
cargo install https://github.com/linyinfeng/think-and-say.git
```

## Usage

```
$ think-and-say --help
think-and-say 0.0.0
Lin Yinfeng <lin.yinfeng@outlook.com>


USAGE:
    think-and-say [OPTIONS] <text>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --aspect-ratio <aspect-ratio>                         [default: 4]
        --min-horizontal-padding <min-horizontal-padding>     [default: 2]
        --min-vertical-padding <min-vertical-padding>         [default: 1]

ARGS:
    <text>    text to be said

$ think-and-say '虽然我其实我并不懂这个梗，但这并不妨碍我造轮子'
****************
********-------*********
*****/          \*********** +-------------------+
*****            \***********|                   |
****/    /--   /--|**********|  虽然我其实我并   |
****|     @     @ |*********/|  不懂这个梗，但   |
*****\            /******-/     这并不妨碍我造   |
******\      O   /****-------|  轮子             |
*******\        /************|                   |
********\      \*************+-------------------+
********/         \********************************
******/           \ \********************************
*****/             | \**************              *****
****|  |           |  |*************  支離滅裂な  ********
****|  |           |  |*************  思考・発言  *********
****+------------------*************              **********
************************************************************
```
