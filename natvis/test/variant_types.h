#include <QtCore>
#include <QtGui>
#include <QtNetwork>
#include <QtWidgets>

class VariantTypes : public QObject
{
    Q_OBJECT

public:
    struct Test
    {
        int a;
        QString b;
    };

    using QObject::QObject;

    QVariant v_null;
    QVariant v_bool = true;
    QVariant v_int = -10;
    QVariant v_uint = 10u;
    QVariant v_double = 3.1415;
    QVariant v_qChar = 'V';
    QVariant v_qString = QStringLiteral("Hello World!");
    QVariant v_qByteArray = QByteArray("Hello World!");
    QVariant v_nullPtr = QVariant::fromValue(nullptr);
    QVariant v_voidStar = QVariant::fromValue(static_cast<void *>(this));
    QVariant v_long = QVariant::fromValue<long>(42);
    QVariant v_longLong = QVariant::fromValue<long long>(42);
    QVariant v_short = QVariant::fromValue<short>(42);
    QVariant v_char = 'V';
    QVariant v_char16 = u'V';
    QVariant v_char32 = U'V';
    QVariant v_ulong = QVariant::fromValue(42ul);
    QVariant v_ulongLong = QVariant::fromValue<unsigned long long>(42);
    QVariant v_ushort = QVariant::fromValue<unsigned short>(42);
    QVariant v_schar = QVariant::fromValue<signed char>('V');
    QVariant v_uchar = QVariant::fromValue<unsigned char>('V');
    QVariant v_float = 3.1415f;
    QVariant v_qfloat16 = QVariant::fromValue<qfloat16>(3.1415);
    QVariant v_qObjectStar = QVariant::fromValue(static_cast<QObject *>(this));
    QVariant v_qCursor = QCursor();
    QVariant v_qDate = QDate::currentDate();
    QVariant v_qSize = QSize(42, 42);
    QVariant v_qTime = QTime::currentTime();
    QVariant v_qVariantList = QVariantList {
        v_qString,
        v_double,
        v_qDate,
    };
    QVariant v_qPolygon = QPolygon({QPoint(0, 0), QPoint(42, 0), QPoint(42, 42), QPoint(0, 42)});
    QVariant v_qPolygonF = QPolygonF({QPointF(0.5, 0.5), QPointF(4.2, 0.5), QPointF(4.2, 4.2), QPointF(0.5, 4.2)});
    QVariant v_qColor = QColor(Qt::red);
    QVariant v_qColorSpace = QColorSpace();
    QVariant v_qSizeF = QSizeF(4.2, 4.2);
    QVariant v_qRectF = QRectF(5.5, 5.5, 4.2, 4.2);
    QVariant v_qLine = QLine(0, 0, 42, 42);
    // QVariant v_qTextLength = QTextLength(QTextLength::VariableLength);
    QVariant v_qStringList = QStringList({
        "one",
        "two",
        "three",
    });
    QVariant v_qVariantMap = QVariantMap {
        {"string", v_qString},
        {"double", v_double},
        {"rect", v_qDate},
    };
    QVariant v_qVariantHash = QVariantHash {
        {"string", v_qString},
        {"double", v_double},
        {"rect", v_qDate},
    };
    QVariant v_qVariantPair = QVariant::fromValue(QVariantPair(v_qString, v_double));
    QVariant v_qIcon = QIcon();
    QVariant v_qPen = QPen();
    QVariant v_qLineF = QLineF(0., 0., 4.2, 4.2);
    QVariant v_qTextFormat = QTextFormat();
    QVariant v_qRect = QRect(5, 5, 42, 42);
    QVariant v_qPoint = QPoint(5, 42);
    QVariant v_qUrl = QUrl("https://github.com/narnaud/natvis4qt");
    QVariant v_qRegularExpression = QRegularExpression(".*");
    QVariant v_qDateTime = QDateTime::currentDateTime();
    QVariant v_qPointF = QPointF(5., 42.);
    QVariant v_qPalette = QPalette();
    QVariant v_qFont = QFont();
    QVariant v_qBrush = QBrush();
    QVariant v_qRegion = QRegion();
    QVariant v_qBitArray = QBitArray(42);
    QVariant v_qImage = QImage(42, 42, QImage::Format_RGB32);
    QVariant v_qKeySequence = QKeySequence(Qt::CTRL + Qt::Key_Q);
    QVariant v_qSizePolicy = QVariant::fromValue(QSizePolicy(QSizePolicy::Expanding, QSizePolicy::Expanding));
    QVariant v_qPixmap = QPixmap(42, 42);
    QVariant v_qLocale = QLocale();
    QVariant v_qBitmap = QBitmap(42, 42);
    QVariant v_qTransform = QTransform();
    QVariant v_qMatrix4x4 = QMatrix4x4();
    QVariant v_qVector2D = QVector2D(42., 42.);
    QVariant v_qVector3D = QVector3D(42., 42., 42.);
    QVariant v_qVector4D = QVector4D(42., 42., 42., 42.);
    QVariant v_qQuaternion = QQuaternion(42., 42., 42., 42.);
    QVariant v_qEasingCurve = QEasingCurve(QEasingCurve::Linear);
    // QVariant v_qJsonValue = QJsonValue();
    // QVariant v_qJsonObject = QJsonObject();
    // QVariant v_qJsonArray = QJsonArray();
    // QVariant v_qJsonDocument = QJsonDocument();
    // QVariant v_qCborValue = QCborValue();
    // QVariant v_qCborArray = QCborArray();
    // QVariant v_qCborMap = QCborMap();
    // QVariant v_qCborSimpleType = QCborSimpleType(QCborKnownTags::DateTimeString);
    QVariant v_qModelIndex = QModelIndex();
    QVariant v_qPersistentModelIndex = QPersistentModelIndex();
    QVariant v_qUuid = QUuid::createUuid();
    QVariant v_qByteArrayList = QVariant::fromValue(QByteArrayList({
        "one",
        "two",
        "three",
    }));
    QVariant v_qVariant = QVariant();
    QVariant v_userType = QVariant::fromValue(Test {
        42,
        "Hello World!",
    });
};

Q_DECLARE_METATYPE(VariantTypes::Test);
