#include <QtCore>

#define TEST_QMAP 0
#define TEST_QHASH 0
#define TEST_QMULTIHASH 0
#define TEST_QVARIANT_UNKOWN 0
#define TEST_QVARIANT_STATIC_PRIMITIVES 0
#define TEST_QVARIANT_CORE_CLASSES 1

int main(int argc, char *argv[])
{
#if TEST_QMAP
    auto mapString = std::map<int, QString>{{10, "one"}, {20, "two"}, {30, "three"}};
    auto qmapString = QMap<int, QString>{{10, "one"}, {20, "two"}, {30, "three"}};
#endif

#if TEST_QHASH
    auto hashString = std::unordered_map<int, QString>{{10, "one"}, {20, "two"}, {30, "three"}};
    auto qhashString = QHash<int, QString>{{10, "one"}, {20, "two"}, {30, "three"}};
#endif

#if TEST_QMULTIHASH
    auto multihashString = std::unordered_multimap<int, QString>{{10, "one"}, {10, "two"}, {10, "three"}, {20, "four"}};
    auto qmultihashString = QMultiHash<int, QString>{{10, "one"}, {10, "two"}, {10, "three"}, {20, "four"}};
#endif

#if TEST_QVARIANT_UNKOWN
    QVariant v_unknown(QUuid{});
#endif

#if TEST_QVARIANT_STATIC_PRIMITIVES
    QVariant v_null;
    QVariant v_bool = true;
    QVariant v_int = -10;
    QVariant v_uint = 10u;
    QVariant v_longlong = -100000ll;
    QVariant v_ulonglong = 100000ull;
    QVariant v_float = 3.1415;
    QVariant v_double = 1.602'176'634e-19;
    QVariant v_char = QVariant::fromValue(static_cast<char>('V'));
#endif

#if TEST_QVARIANT_CORE_CLASSES
    QVariant v_qchar = QChar('V');
    QVariant v_qstring = "Hello World!";
    QVariant v_qbytearray = QByteArray("Hello World!");
    QVariant v_qdate = QDate::currentDate();
    QVariant v_qtime = QTime::currentTime();
    QVariant v_qdatetime = QDateTime::currentDateTime(); // NOK
    QVariant v_qurl = QUrl("https://www.kdab.com");      // NOK
    QVariant v_qrect = QRect(5, 5, 42, 42);
    QVariant v_qrectf = QRectF(5., 5., 4.2, 4.2); // NOK
    QVariant v_size = QSize(5, 42);
    QVariant v_sizef = QSizeF(5., 4.2); // NOK
    QVariant v_qline = QLine(5, 5, 42, 42);
    QVariant v_qlinef = QLineF(5., 5., 4.2, 4.2); // NOK
    QVariant v_qpoint = QPoint(5, 42);
    QVariant v_qpointf = QPointF(5., 4.2);
#endif

    return 0;
}
