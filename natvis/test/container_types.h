#include <QtCore>

#include <unordered_map>
#include <unordered_set>

class ContainerTypes : public QObject
{
    Q_OBJECT

public:
    using QObject::QObject;

    QByteArrayList qByteArrayList = {
        "one",
        "two",
        "three",
    };
    QHash<int, QString> qHash = {
        {10, "one"},
        {20, "two"},
        {30, "three"},
    };
    QMap<int, QString> qMap = {
        {10, "one"},
        {20, "two"},
        {30, "three"},
    };
    QMultiHash<int, QString> qMultiHash = {
        {10, "one"},
        {10, "two"},
        {10, "three"},
        {20, "four"},
    };
    QSet<QString> qSet = {
        "one",
        "two",
        "three",
        "four",
    };
    QStringList qStringList = {
        "one",
        "two",
        "three",
    };
    QVarLengthArray<QString, 4> qVarLengthArray = {
        "one",
        "two",
        "three",
        "four",
    };

    std::map<int, QString> stdMap = {
        {10, "one"},
        {20, "two"},
        {30, "three"},
    };
    std::unordered_map<int, QString> stdUnorderedMap = {
        {10, "one"},
        {20, "two"},
        {30, "three"},
    };
    std::unordered_multimap<int, QString> stdUnorderedMultimap = {
        {10, "one"},
        {10, "two"},
        {10, "three"},
        {20, "four"},
    };
    std::unordered_set<std::string> stdSet = {
        "one",
        "two",
        "three",
        "four",
    };
};
