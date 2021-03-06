# Enayet Shell
| master branch build status | develop branch build status | codecov status |
|:-:|:-:|:-:|
| [![Build Status](https://travis-ci.org/afnanenayet/Enayet-Shell.svg?branch=master)](https://travis-ci.org/afnanenayet/Enayet-Shell) | [![Build Status](https://travis-ci.org/afnanenayet/Enayet-Shell.svg?branch=develop)](https://travis-ci.org/afnanenayet/Enayet-Shell) | [![codecov](https://codecov.io/gh/afnanenayet/Enayet-Shell/branch/master/graph/badge.svg)](https://codecov.io/gh/afnanenayet/Enayet-Shell) |

The Enayet Shell is a pet project so that I can get familiar with Rust, and
have fun creating a shell (something I've been meaning to get to but never
quite got the chance to do). This is a fairly minimal shell but I hope to
add on to it as time goes on. I will also rewrite small shell utilities
in Rust (such as `cd` and `ls`).

# Building the shell

I built this on MacOS 10.12, 10.13 and on Linux (Ubuntu Precise via Travis CI).
It has not been tested on any other platform. It's probably not going to work
on Windows.

To modify or build this shell:

    git clone https://github.com/afnanenayet/Enayet-Shell.git
    cargo build --release

Note that the shell must be built with Cargo so that the program knows what
version it is (shell output relies on a cargo variable)

If you want to build the compressed version of the shell (using gzexe):

    git clone https://github.com/afnanenayet/Enayet-Shell.git
    ./build_release.sh

## Dependencies

In order to create the shell, you will need to have the Rust language installed.
If you want to create the compressed version of the shell, you will need the
following dependencies installed and available in your `PATH`:

- gzip
- gzexe
- basename
- chmod
- ln
- mkdir
- mktemp
- rm
- sleep
- tail

The development of this shell also depends on `git-flow`. The development branch
is called `develop`.

# Usage
The shell will be built as an executable. To run the shell

    ./ensh [config_path]

## Arguments
* `config_path` - the file path to the configuration file for the shell. The
the default config file is named `.ensh_config`. The system looks for
`~/.ensh_config` by default, but an alternate path can name any file in any
location, as long as it conforms to the format and is reachable by the shell
binary.

# Configuration
The config file has a very simple format that is subject to change with any
update. Currently, the config file contains path directories, separated by
lines. The config file may contain nonexistent directories - this will not
throw an error, the shell will search for executables in the directories it
can find.

The default config file includes the following paths:

    /usr/bin
    /usr/local/bin
    /usr/local/sbin
    /bin
    /usr/sbin

# Roadmap 2018
- [ ] tab completion
- [ ] style customization via config file
- [ ] python/python style scripting
