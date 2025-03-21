#include <QtCore>

class CoreTypes : public QObject
{
    Q_OBJECT

public:
    enum SelectionFlag {
        NoUpdate = 0x0000,
        Clear = 0x0001,
        Select = 0x0002,
        Deselect = 0x0004,
        Toggle = 0x0008,
        Current = 0x0010,
        Rows = 0x0020,
        Columns = 0x0040,
        SelectCurrent = Select | Current,
        ToggleCurrent = Toggle | Current,
        ClearAndSelect = Clear | Select
    };
    Q_DECLARE_FLAGS(SelectionFlags, SelectionFlag)
    Q_FLAG(SelectionFlags)

    using QObject::QObject;

    QByteArray qByteArray = QByteArray("Hello World!");
    QChar qChar = QChar('c');
    QDate qDate = QDate::currentDate();
    QDateTime qDateTime = QDateTime::currentDateTime();
    QDir qDir = QDir::currentPath();
    QFile qFile = QFile(QCoreApplication::applicationFilePath());
    QFileInfo qFileInfo = QFileInfo(QCoreApplication::applicationFilePath());
    SelectionFlags qFlags = SelectionFlag::SelectCurrent;
    QLine qLine = QLine(0, 0, 42, 42);
    QPoint qPoint = QPoint(24, 48);
    QPointF qPointF = QPointF(24.5, 48.5);
    QRect qRect = QRect(5, 5, 42, 42);
    QRectF qRectF = QRectF(5.5, 5.5, 4.2, 4.2);
    QSize qSize = QSize(42, 42);
    QSizeF qSizeF = QSizeF(4.2, 4.2);
    QString qString = QString("Hello World!");
    QStringView qStringView = QStringView(qString);
    QTime qTime = QTime::currentTime();
    QUrl qUrl = QUrl("https://github.com/narnaud/natvis4qt");
    QUuid qUuid = QUuid::createUuid();
};
