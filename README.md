Command line [hacker news](https://news.ycombinator.com) written in rust

Very minimal (literally 80 lines of code). Very scrappy. I wrote it so that I can display top hacker news articles in [Conky](https://github.com/brndnmtthws/conky)

### Installation

`cargo install rustynews` and wait till the dependencies compile.

### Usage

```
USAGE:
    rustynews [FLAGS] <num>

FLAGS:
    -b, --by          Display username
    -h, --help        Prints help information
    -p, --progress    Show progress bar while posts are being fetched
    -s, --score       Display score
    -u, --url         Display URL
    -V, --version     Prints version information

ARGS:
    <num>    The number of top posts to be retrieved
```


 
![](rusty.gif)
