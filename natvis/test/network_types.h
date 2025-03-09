#include <QtNetwork>

class NetworkTypes : public QObject
{
    Q_OBJECT

public:
    using QObject::QObject;

    QHostAddress qHostAddress = QHostAddress(QHostAddress::LocalHost);
    QHostAddress qHostAddressV4 = QHostAddress("76.124.119.133");
    QHostAddress qHostAddressV6 = QHostAddress("2001:0db8:85a3:0000:0000:8a2e:0370:7334");
    QHostInfo qHostInfo = QHostInfo();
};
