# Nativs4Qt - Install natvis files for Qt

This repository contains:

- Up to date natvis files for Qt5 and Qt6
- Command line tool to install or update them in different locations

## Natvis4Qt

`natvis4qt` is a simple command line tool to install or update the natvis files in different locations.

> [!CAUTION]
> The tool is in early stage, for now it can only overwrite natvis file in known locations. More later!

### Installation via [Scoop](https://scoop.sh/) (preferred)

```batch
scoop bucket add narnaud https://github.com/narnaud/scoop-bucket
scoop install natvis4qt
```

During a scoop update, installed Natvis files will be automatically updated. If you don't want this mechanism, run:

```cmd
natvis4qt set --autoupdate false
```

### Or via archive files

1. Go to the [Releases](https://github.com/narnaud/natvis4qt/releases) page
2. Download the latest `natvis-x86_64-pc-windows-msvc.zip` file
3. Extract the files from it into a directory.

### Usage

Just run `natvis4qt install` on the command line and follow the instructions.

![Demo](assets/demo.gif)

The application is using different commands:

- `install`  Install the natvis files in known directories (MSVC and Qt)
- `update`   Update the natvis files
- `set`      Adjust natvis4qt's settings

## Integration

### Visual Studio

Normally you don't have to do anything, the natvis files are copied in a central place that is used by Visual Studio.

See documentation here: [Natvis file locations](https://learn.microsoft.com/en-us/visualstudio/debugger/create-custom-views-of-native-objects?view=vs-2022#BKMK_natvis_location)

### Visual Studio Code

In order to use this natvis file in VS Code, you need to add a `visualizerFile` to your launch configuration. Edit your `launch.json` file (or the `launch` section of your `*.code-workspace` file) and add something like that:

```json
    "configurations": [
        {
            "name": "Current Target (VS)",
            "type": "cppvsdbg",
            "request": "launch",
            "program": "${command:cmake.launchTargetPath}",
            "visualizerFile": "${env:QTDIR}/natvis/qt6.natvis",
            "cwd": "${workspaceFolder}",
            "sourceFileMap": {
                "C:/work/build/qt5_workdir/w/s": "${env:QTDIR}/../Src",
                "Q:/qt5_workdir/w/s": "${env:QTDIR}/../Src",
                "C:/Users/qt/work/install": "${env:QTDIR}/../Src",
                "C:/Users/qt/work/qt": "${env:QTDIR}/../Src"
            }
        }
    ]
```

> [!IMPORTANT]
> The `QTDIR` environment variable must be set up before launching VS Code

## Natvis Files

### Qt 5

The `qt5.natvis` file is the one used in the [Qt VS Addin](<https://wiki.qt.io/Visual_Studio_Add-in>) from The Qt Company.

### Qt 6

The `qt6.natvis` file is the one used in the [Qt VS Addin](<https://wiki.qt.io/Visual_Studio_Add-in>) from The Qt Company.

`qt6-extension.natvis` are external contributions, with proper copyright attribution.

> ![IMPORTANT]
> The 2 files will be merge during the release, so you only need to deal with one `qt6.natvis`.
> This is done like this here to ease update of `qt6.natvis`.

## License

The **natvis4qt** tool is licensed under the MIT license.

`qt5.natvis`:

- [The Qt Company](https://www.qt.io/): licensed Qt-Commercial OR GPL-3.0-only WITH Qt-GPL-exception-1.0

`qt6.natvis`:

- [The Qt Company](https://www.qt.io/): licensed Qt-Commercial OR GPL-3.0-only WITH Qt-GPL-exception-1.0
- [QMap](https://github.com/qt-labs/vstools/commit/97dd70cd5b1c3c9a310377f03bf2a989d60bb1b1), [QHash](https://github.com/qt-labs/vstools/commit/71e0e9e7fecc6c1077c90a5ec739f5d89dcf5fa5), [QSet](https://github.com/qt-labs/vstools/commit/86270320212a8a9c7d3749613c4b5c189e2569fa) and [QVariant](https://github.com/qt-labs/vstools/commit/d21e92652c9728fb0512813f6938588b16ac39d1) are copyright [Klarälvdalens Datakonsult AB (KDAB)](https://www.kdab.com/)

`qt6-extension.natvis`:

- [@nholthaus](https://github.com/nholthaus): licensed MIT
- [Klarälvdalens Datakonsult AB (KDAB)](https://www.kdab.com/): licensed MIT
