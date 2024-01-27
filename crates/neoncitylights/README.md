# neoncitylights/cli

## Configure settings
```shell
# set your global default settings
# neoncitylights config set <key> <value>
neoncitylights config set author "Samantha Nguyen"
neoncitylights config set email contact@samanthanguyen.me
neoncitylights config set license MIT
neoncitylights config set license "MIT or APACHE 2.0"
neoncitylights config set git-init yes

# retrieve all settings
neoncitylights config list

# get value for a config setting key
# neoncitylights config get <key>
neoncitylights config get license # contact@samanthanguyen.me
```

## Create a project
```shell
neoncitylights new
    --language {rs|ts|php|c|cpp|...}
    --kind {app|bin|executable|...}
```
