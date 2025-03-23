# Natvis4Qt - Qt support

Show the current status of different classes. If you are interested in one missing here, feel free to create an issue here: <https://github.com/narnaud/natvis4qt/issues>

## Qt6 natvis

```
✅ Tested
✔️ Untested
❌ Does not work
```

|                                  | Version | Qt6.8.2 |
| -------------------------------- | ------- | ------- |
| **CORE TYPES**                   |         |         |
| QByteArray                       | 0.1     | ✅      |
| QCBor*[^1]                       | 0.5     | ✔️    |
| QChar                            | 0.1     | ✅      |
| QDate                            | 0.2     | ✅      |
| QDateTime[^1] [^2]               | 0.6     | ✅      |
| QDir                             | 0.4     | ✅      |
| QFile                            | 0.4     | ✅      |
| QFileInfo                        | 0.4     | ✅      |
| QFlags                           | 0.4     | ✅      |
| QJson*[^1]                       | 0.5     | ✅      |
| QLine                            | 0.1     | ✅      |
| QModelIndex                      |         | ❌      |
| QObject                          |         | ❌      |
| QPoint                           | 0.1     | ✅      |
| QPersistentModelIndex            |         | ❌      |
| QPropertyData                    | 0.1     | ✔️    |
| QRect                            | 0.1     | ✅      |
| QRectF                           | 0.1     | ✅      |
| QRegularExpression               |         | ❌      |
| QSize                            | 0.1     | ✅      |
| QSizeF                           | 0.1     | ✅      |
| QString                          | 0.1     | ✅      |
| QStringView                      | 0.1     | ✅      |
| QTime                            | 0.1     | ✅      |
| QUrl                             | 0.1     | ✅      |
| QUuid                            | 0.1     | ✅      |
| **CONTAINER TYPES**              |         |         |
| QByteArrayList                   | 0.1     | ✅      |
| QHash                            | 0.1     | ✅      |
| QList                            | 0.1     | ✅      |
| QMap                             | 0.1     | ✅      |
| QMultiHash                       | 0.1     | ✅      |
| QPair                            | 0.1     | ✅      |
| QSet                             | 0.1     | ✅      |
| QStringList                      | 0.1     | ✅      |
| QVarLengthArray                  | 0.1     | ✅      |
| **QVARIANT TYPES**               |         |         |
| Null variant                     | 0.1     | ✅      |
| QMetaType::Void                  | 0.1     | ✅      |
| QMetaType::Bool                  | 0.1     | ✅      |
| QMetaType::Int                   | 0.1     | ✅      |
| QMetaType::UInt                  | 0.1     | ✅      |
| QMetaType::Double                | 0.1     | ✅      |
| QMetaType::QChar                 | 0.1     | ✅      |
| QMetaType::QString               | 0.1     | ✅      |
| QMetaType::QByteArray            | 0.1     | ✅      |
| QMetaType::Nullptr               | 0.1     | ✅      |
| QMetaType::VoidStar              | 0.1     | ✅      |
| QMetaType::Long                  | 0.1     | ✅      |
| QMetaType::LongLong              | 0.1     | ✅      |
| QMetaType::Short                 | 0.1     | ✅      |
| QMetaType::Char                  | 0.1     | ✅      |
| QMetaType::Char16                | 0.1     | ✅      |
| QMetaType::Char32                | 0.1     | ✅      |
| QMetaType::ULong                 | 0.1     | ✅      |
| QMetaType::ULongLong             | 0.1     | ✅      |
| QMetaType::UShort                | 0.1     | ✅      |
| QMetaType::SChar                 | 0.1     | ✅      |
| QMetaType::UChar                 | 0.1     | ✅      |
| QMetaType::Float                 | 0.1     | ✅      |
| QMetaType::Float16               | 0.1     | ✅      |
| QMetaType::QObjectStar           |         | ❌      |
| QMetaType::QCursor               |         | ❌      |
| QMetaType::QDate                 | 0.1     | ✅      |
| QMetaType::QSize                 | 0.1     | ✅      |
| QMetaType::QTime                 | 0.1     | ✅      |
| QMetaType::QVariantList          | 0.1     | ✅      |
| QMetaType::QPolygon              |         | ❌      |
| QMetaType::QPolygonF             |         | ❌      |
| QMetaType::QColor                |         | ❌      |
| QMetaType::QColorSpace           |         | ❌      |
| QMetaType::QSizeF                | 0.1     | ✅      |
| QMetaType::QRectF                | 0.1     | ✅      |
| QMetaType::QLine                 | 0.1     | ✅      |
| QMetaType::QTextLength           |         | ❌      |
| QMetaType::QStringList           | 0.1     | ✅      |
| QMetaType::QVariantMap           | 0.1     | ✅      |
| QMetaType::QVariantHash          | 0.1     | ✅      |
| QMetaType::QVariantPair          |         | ❌      |
| QMetaType::QIcon                 |         | ❌      |
| QMetaType::QPen                  |         | ❌      |
| QMetaType::QLineF                | 0.1     | ✅      |
| QMetaType::QTextFormat           |         | ❌      |
| QMetaType::QRect                 | 0.1     | ✅      |
| QMetaType::QPoint                | 0.1     | ✅      |
| QMetaType::QUrl                  |         | ❌      |
| QMetaType::QRegularExpression    |         | ❌      |
| QMetaType::QDateTime             |         | ❌      |
| QMetaType::QPointF               | 0.1     | ✅      |
| QMetaType::QPalette              |         | ❌      |
| QMetaType::QFont                 |         | ❌      |
| QMetaType::QBrush                |         | ❌      |
| QMetaType::QRegion               |         | ❌      |
| QMetaType::QBitArray             |         | ❌      |
| QMetaType::QImage                |         | ❌      |
| QMetaType::QKeySequence          |         | ❌      |
| QMetaType::QSizePolicy           |         | ❌      |
| QMetaType::QPixmap               |         | ❌      |
| QMetaType::QLocale               |         | ❌      |
| QMetaType::QBitmap               |         | ❌      |
| QMetaType::QTransform            |         | ❌      |
| QMetaType::QMatrix4x4            |         | ❌      |
| QMetaType::QVector2D             |         | ❌      |
| QMetaType::QVector3D             |         | ❌      |
| QMetaType::QVector4D             |         | ❌      |
| QMetaType::QQuaternion           |         | ❌      |
| QMetaType::QEasingCurve          |         | ❌      |
| QMetaType::QJsonValue            |         | ❌      |
| QMetaType::QJsonObject           |         | ❌      |
| QMetaType::QJsonArray            |         | ❌      |
| QMetaType::QJsonDocument         |         | ❌      |
| QMetaType::QCborValue            |         | ❌      |
| QMetaType::QCborArray            |         | ❌      |
| QMetaType::QCborMap              |         | ❌      |
| QMetaType::QCborSimpleType       |         | ❌      |
| QMetaType::QModelIndex           |         | ❌      |
| QMetaType::QPersistentModelIndex |         | ❌      |
| QMetaType::QUuid                 |         | ❌      |
| QMetaType::QByteArrayList        | 0.1     | ✅      |
| QMetaType::QVariant              |         | ❌      |
| QMetaType::User                  |         | ❌      |
| **NETWORK TYPES**                |         |         |
| QHostAddress                     | 0.2     | ✅      |
| QHostInfo                        |         | ❌      |
| **GUI TYPES**                    |         |         |
| QBitmap                          | 0.2     | ✅      |
| QBrush                           |         | ❌      |
| QCursor                          |         | ❌      |
| QFont                            |         | ❌      |
| QImage                           | 0.2     | ✅      |
| QKeySequence                     |         | ❌      |
| QMatrix4x4                       | 0.1     | ✅      |
| QPalette                         |         | ❌      |
| QPen                             |         | ❌      |
| QPicture                         |         | ❌      |
| QPixmap                          | 0.2     | ✅      |
| QPolygon                         | 0.1     | ✅      |
| QPolygonF                        | 0.1     | ✅      |
| QQuaternion                      | 0.1     | ✅      |
| QRegion                          |         | ❌      |
| QTransform                       |         | ❌      |
| QVector2D                        | 0.1     | ✅      |
| QVector3D                        | 0.1     | ✅      |
| QVector4D                        | 0.1     | ✅      |
| **WIDGET TYPES**                 |         |         |
| QSizePolicy                      | 0.1     | ✅      |
| QWidget                          |         | ❌      |
| **QML TYPES**                    |         |         |
| QQuickItem                       | 0.1     | ✔️    |

[^1]: Only in dynamic debug builds of Qt
[^2]: This visualizer is complex. You might need to bump the recursion limit in the Visual Studio settings. There's no known setting for VS Code.
