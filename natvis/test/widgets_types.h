#include <QtWidgets>

class WidgetsTypes : public QObject
{
    Q_OBJECT

public:
    using QObject::QObject;

    QSizePolicy qSizePolicy = QSizePolicy(QSizePolicy::Expanding, QSizePolicy::Expanding);
    QWidget qWidget;
};
