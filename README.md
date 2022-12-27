<div align="center">

# Configz

Simple configuration management tool written in rust and configured with lua

</div>

Configz is designed to be a simple but powerful way to configure a server or
local machine. It is distributed as a single binary and configuration is
written in the Lua scripting language. It provides a minimal standard library
to manage files and run commands on a machine. Configz aims to be the middle
ground between bash scripts and full-blown automation tools like Ansible or
Puppet.

## Installing

Currently there is no script for installing, you can manually download the
release binary from the
[releases](https://github.com/AdeAttwood/Configz/releases) page.

## Getting started

1) Create the directory structure for your project that will typically be
stored in source control.

```sh
mkdir -p configz-test/{files,modules}
cd configz-test
```



2) Add your first configuration file in the `files` directory

```toml
# files/gitconfig
[user]
  name = My Name
  email = me@example.com
```

3) Create your first configz module in the `modules` folder

```lua
-- modules/git.lua
local home = os.getenv("HOME")
configz.file(home .. "/.gitconfig", { source = "files/gitconfig" })
```

4) Apply your configuration module

```sh
configz --module modules/git
```

For inspiration on creating modules, you can reference the
[`lua`](https://github.com/AdeAttwood/Configz/tree/0.x/lua) directory in the
repo, this has some examples and over time will hopefully become a bit of a
standard library.

## Lua API

Full the full documented api please reference the [lua
definitions](https://github.com/AdeAttwood/Configz/blob/0.x/definitions/configz.lua)


