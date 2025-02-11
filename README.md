# PCF Version Incrementer

> [!WARNING]
> This project is no longer maintained. Visit [https://github.com/felixnhs/pcf-version-incrementer-go](https://github.com/felixnhs/pcf-version-incrementer-go) instead.

Simple command line tool for incrementing the [PCF](https://learn.microsoft.com/en-us/power-apps/developer/component-framework/overview)-controls version number.




## Usage/Examples

#### Status
Show the current version of your component.
```
pfcv status
```

Use the `-v` flag to show the versions specified in all relevant files.

#### Increment
Increments the minor version by one.
```
pfcv increment
```
Use the `--major` flag to increment the major version instead.


#### Set
Set a specific version.
```
pfcv set --major 2 --minor 1 --patch 0
```

#### Help
Show help about the tool or about a specific command.
```
pfcv help [COMMAND]
```


## FAQ

#### Why though? Seems pointless...

For some reason the componentes I've beed working on didn't update when importing a solution/component with the same version. Since I'm too lazy to manually update the version in three places, I automated the 10sec task in just one Day! Seriously though, it's just nice not to worry about that.



## License

See [MIT License](LICENSE.md)

