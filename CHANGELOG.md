# Changelog

## [0.5.1](https://github.com/narnaud/natvis4qt/compare/v0.5.0...v0.5.1) (2025-03-31)


### Bug Fixes

* Change qt*.natvis license ([ab1061f](https://github.com/narnaud/natvis4qt/commit/ab1061f3f84bdbb3358c203be041ed5f439db5f8))
* The Image Watch visualizer must be declared first ([4446d38](https://github.com/narnaud/natvis4qt/commit/4446d3890d5e6a4c05f9e9996185c591626e27e1))

## [0.5.0](https://github.com/narnaud/natvis4qt/compare/v0.4.0...v0.5.0) (2025-03-30)


### Features

* add Image Watch support for QPixmap/QImage ([b365a09](https://github.com/narnaud/natvis4qt/commit/b365a09d6eebfa7166f48d4616e379f52f57d7db))
* Move VS Code natvis up in the list ([595b4bc](https://github.com/narnaud/natvis4qt/commit/595b4bc8c398086480832930d4c5d3ae246809b2))
* **natvis:** Add QJson support from Nerixyz ([61cf840](https://github.com/narnaud/natvis4qt/commit/61cf840129d4eb9a82dcbe5821c1f6f04bfd9480)), closes [#12](https://github.com/narnaud/natvis4qt/issues/12)
* **natvis:** More QVariant visualizers ([17b57c8](https://github.com/narnaud/natvis4qt/commit/17b57c8f5d5bfb0d78418192ec8d76ebd7bafc01))
* **natvis:** visualize `QDateTime` ([a83155a](https://github.com/narnaud/natvis4qt/commit/a83155ac1f5da443c205a390b854a7b95076bdbf))


### Bug Fixes

* **natvis:** Remove reference count for QPolygon* ([da5024a](https://github.com/narnaud/natvis4qt/commit/da5024a84a97f22b63633134aa4aaedf8e62509e))
* **natvis:** various fixes for Qt 6 and 6.9.0 compat ([f168fe8](https://github.com/narnaud/natvis4qt/commit/f168fe8885cb3bd4b3c3532d1f19c99a17da848e))
* **qmap:** Order `Intrinsic`s correctly for 6.9.0 ([f3d438f](https://github.com/narnaud/natvis4qt/commit/f3d438f02753845d341aa3e1ae9e513a6cc06b08))
* Qt 6.9.0 compat ([f9bfcd5](https://github.com/narnaud/natvis4qt/commit/f9bfcd5f69c717d47557440402fd1947253552a8))

## [0.4.0](https://github.com/narnaud/natvis4qt/compare/v0.3.1...v0.4.0) (2025-03-09)


### Features

* **app:** add vscode cpptools directory ([7aa7e97](https://github.com/narnaud/natvis4qt/commit/7aa7e978d6c237cd4583c66665526722d361a429))
* **app:** Enhance install to merge with exiting installation ([5bc8f8c](https://github.com/narnaud/natvis4qt/commit/5bc8f8c80eafe8c771e2286cd6baa125780085b6))
* **natvis:** add `QFlags`, `QDir`, `QFileInfo`, and `QFile` ([b4cb221](https://github.com/narnaud/natvis4qt/commit/b4cb221429aa054b2ef33560a91de87d746b7619))


### Bug Fixes

* **app:** Fix VS 2019 key ([8b86b7e](https://github.com/narnaud/natvis4qt/commit/8b86b7e7d230ccf7ea01af5c01e7efc757fc05dc))
* **builder:** Fix builder to only assemble natvis files in the input directory ([28809ce](https://github.com/narnaud/natvis4qt/commit/28809ce5ed50ce45d9e14675938c1f53fdc67ffd))

## [0.3.1](https://github.com/narnaud/natvis4qt/compare/v0.3.0...v0.3.1) (2025-03-06)


### Bug Fixes

* **app:** Fix typo in help ([381e2c2](https://github.com/narnaud/natvis4qt/commit/381e2c2a9ff1ed1db07497b9011035c5f54cce12))

## [0.3.0](https://github.com/narnaud/natvis4qt/compare/v0.2.0...v0.3.0) (2025-03-05)


### Features

* **app:** Add autoupdate as a subcommand ([218fa7a](https://github.com/narnaud/natvis4qt/commit/218fa7a2e2fae351ec812ea4224380bc7ee31964))
* **app:** Add install and update subcommands ([2976ad4](https://github.com/narnaud/natvis4qt/commit/2976ad457f6ebbb18d3be379d98335ef8967a197))
* **app:** Save Qt root folder in app persistent settings ([121dd58](https://github.com/narnaud/natvis4qt/commit/121dd5848adefbc6adc7c7d5f84e9476749e3ab5))
* **app:** Store selected installs in app persistent settings ([3be2dec](https://github.com/narnaud/natvis4qt/commit/3be2dec5bf1c8d04789f2a9f310e10977768f51a))


### Bug Fixes

* **builder:** Fix for more than 2 files ([700f019](https://github.com/narnaud/natvis4qt/commit/700f019d522837a1e2de5b7d43ea0cf67a680a30))
* **builder:** Oups, forgot to rename after extracting the function ([f36fbc2](https://github.com/narnaud/natvis4qt/commit/f36fbc27bcea4478480cd35babf39eafbbe48c0d))

## [0.2.0](https://github.com/narnaud/natvis4qt/compare/v0.1.0...v0.2.0) (2025-03-03)


### Features

* Add natvis merger ([11a3680](https://github.com/narnaud/natvis4qt/commit/11a36804ce481f17eb09604c6b08d1e4264420d1))
* Create a builder for natvis files ([990fe09](https://github.com/narnaud/natvis4qt/commit/990fe095dc24191bd655a7ff6d98efb841b9c3dd))
* **qt6:** Add better QDate vizualiser ([976e5db](https://github.com/narnaud/natvis4qt/commit/976e5db1ea2b423efbe70de9a0b9b16cfcc8eef2))
* **qt6:** Add QHostAddress visualizer ([8c77b9e](https://github.com/narnaud/natvis4qt/commit/8c77b9e8c33d63eb819128f3e5071bed97876bdc))
* **qt6:** Add QImage and QPixmap ([4b0d7c8](https://github.com/narnaud/natvis4qt/commit/4b0d7c835a8f29f4b59e719009ceb08e2501aeed))
* Restart from TQC natvis files ([39feb31](https://github.com/narnaud/natvis4qt/commit/39feb315d1948c4f9efc2fcbdea77cd54e77946f))


### Bug Fixes

* Fix deploy script hopefully ([9ee3bba](https://github.com/narnaud/natvis4qt/commit/9ee3bba5513da04ac2f66d366bbf6c3e77065e6c))

## 0.1.0 (2025-03-02)


### Features

* Add rust application to list possible target directories ([0b65d1a](https://github.com/narnaud/natvis4qt/commit/0b65d1a549441d25e11c1ae48c457f66d12bc974))
* Add validation for the Qt install root path ([10108a1](https://github.com/narnaud/natvis4qt/commit/10108a1e7a8d880fb3dd5964f5f762f90e4dc1df))
* Ask Qt install root if it's not the default one ([e0c765a](https://github.com/narnaud/natvis4qt/commit/e0c765a424d1f7a6086ac9838981c5b02f211289))
* Copy natvis file to selected directories ([671bf5d](https://github.com/narnaud/natvis4qt/commit/671bf5d2523d2f0520f7fcd739c0809df7884fc7))
* **qt5:** Add Qt5 natvis file ([2cfa744](https://github.com/narnaud/natvis4qt/commit/2cfa74400f441ecd06f843d4f15e428287c3bc66))
* **qt6:** Add Qt6 natvis ([47132b1](https://github.com/narnaud/natvis4qt/commit/47132b1eb09c9603948addcd8575e2ee4f48578f))


### Bug Fixes

* Issue when multiple Qt architecture for the same version is installed ([52c734d](https://github.com/narnaud/natvis4qt/commit/52c734d2e247ff22dc618ed438a44a3298bac2b1))
