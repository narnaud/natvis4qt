# Nativs4Qt - Install natvis files for Qt

This repository contains:

- Up to date natvis files for Qt5 and Qt6
- Command line tool to install or update them in different locations

## Natvis4Qt

**Natvis4Qt` is a simple command line tool to install or update the natvis files in different locations.

> [!CAUTION]
> The tool is in early stage, for now it can only overwrite natvis file in known locations. More later!

### Installation via [Scoop](https://scoop.sh/)

```
scoop bucket add narnaud https://github.com/narnaud/scoop-bucket
scoop install natvis4qt
```

### Or via archive files

1. Go to the [Releases](https://github.com/narnaud/natvis4qt/releases) page
2. Download the latest `natvis-x86_64-pc-windows-msvc.zip` file
3. Extract the files from it into a directory.

### Usage

Just start `natvis4qt` on the command line and follow the instructions.

![Demo](assets/demo.gif)

## Integration

### Visual Studio

Normally you don't have to do anything, the natvis files are copied in a central place that is used by Visual Studio.

See documentation here: [Natvis file locations](https://learn.microsoft.com/en-us/visualstudio/debugger/create-custom-views-of-native-objects?view=vs-2022#BKMK_natvis_location)

### Visual Studio Code

In order to use this natvis file in VS Code, you need to add a `visualizerFile` to your launch configuration. Edit your `launch.json` file (or the `launch` section of your `*.code-workspace` file) and add something like that:

```
    "configurations": [
        {
            "name": "Current Target (VS)",
            "type": "cppvsdbg",
            "request": "launch",
            "program": "${command:cmake.launchTargetPath}",
            "visualizerFile": "${QTDIR}/natvis/qt6.natvis",
            "cwd": "${workspaceFolder}",
            "sourceFileMap": {
                "C:/work/build/qt5_workdir/w/s": "${QTDIR}/../Src",
                "Q:/qt5_workdir/w/s": "${QTDIR}/../Src",
                "C:/Users/qt/work/install": "${QTDIR}/../Src",
                "C:/Users/qt/work/qt": "${QTDIR}/../Src"
            }
        }
    ]
```

> [!IMPORTANT]
> The `QTDIR` environment variable must be set up before launching VS Code

## Natvis Files

### Qt 5

The natvis file is the one used in the [Qt VS Addin](<https://wiki.qt.io/Visual_Studio_Add-in>) from The Qt Company.

### Qt 6

The natvis file started as the one used the [Qt VS Addin](<https://wiki.qt.io/Visual_Studio_Add-in>) from The Qt Company.

External contributions are marked explicitly in the header of the `qt6.natvis` file, with proper copyright attribution.

## License

The **natvis4qt** tool is licensed under the MIT license.

The `qt5.natvis` file is licensed Qt-Commercial OR GPL-3.0-only WITH Qt-GPL-exception-1.0.

The `qt6.natvis` file is a collection of multiple contributions:

- [The Qt Company](https://www.qt.io/): licensed Qt-Commercial OR GPL-3.0-only WITH Qt-GPL-exception-1.0
- [Klarälvdalens Datakonsult AB (KDAB)](https://www.kdab.com/): licensed MIT
- [@nholthaus](https://github.com/nholthaus): licensed MIT
