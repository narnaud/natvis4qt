#include <QtCore>

int main(int argc, char *argv[])
{
    auto vecString = std::vector<std::string>{"one", "two", "three"};
    auto qvecString = QVector<QString>{"one", "two", "three"};

    auto mapString = std::map<int, QString>{{1, "one"}, {2, "two"}, {3, "three"}};
    auto qmapString = QMap<int, QString>{{1, "one"}, {2, "two"}, {3, "three"}};

    return 0;
}
