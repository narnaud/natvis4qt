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

    return 0;
}
