#include <QtCore>

int main(int argc, char *argv[])
{
    auto vecString = std::vector<std::string>{"one", "two", "three"};
    auto qvecString = QVector<QString>{"one", "two", "three"};

    auto mapString = std::map<int, QString>{{10, "one"}, {20, "two"}, {30, "three"}};
    auto qmapString = QMap<int, QString>{{10, "one"}, {20, "two"}, {30, "three"}};

    auto hashString = std::unordered_map<int, QString>{{10, "one"}, {20, "two"}, {30, "three"}};
    auto qhashString = QHash<int, QString>{{10, "one"}, {20, "two"}, {30, "three"}};

    auto multihashString = std::unordered_multimap<int, QString>{{10, "one"}, {10, "two"}, {10, "three"}, {20, "four"}};
    auto qmultihashString = QMultiHash<int, QString>{{10, "one"}, {10, "two"}, {10, "three"}, {20, "four"}};

    QVariant v_null;
    QVariant v_bool = true;
    QVariant v_int = -10;
    QVariant v_uint = 10u;
    QVariant v_longlong = -100000ll;
    QVariant v_ulonglong = 100000ull;
    QVariant v_float = 3.1415;
    QVariant v_double = 1.602'176'634e-19;
    QVariant v_char = QVariant::fromValue(static_cast<char>('V'));

    QVariant v_unknown(QRect{});
    return 0;
}
