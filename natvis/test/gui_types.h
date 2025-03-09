#include <QtGui>

class GuiTypes : public QObject
{
    Q_OBJECT

public:
    using QObject::QObject;

    QBitmap qBitmap = QBitmap(":/test/qt-logo.png");
    QBrush qBrush = QBrush();
    QCursor qCursor = QCursor();
    QFont qFont = QFont();
    QImage qImage = QImage(":/test/qt-logo.png");
    QKeySequence qKeySequence = QKeySequence(Qt::CTRL + Qt::Key_Q);
    QMatrix4x4 qMatrix4x4 = QMatrix4x4();
    QPalette qPalette = QPalette();
    QPen qPen = QPen();
    QPicture qPicture = QPicture();
    QPixmap qPixmap = QPixmap(":/test/qt-logo.png");
    QPolygon qPolygon = QPolygon({QPoint(0, 0), QPoint(42, 0), QPoint(42, 42), QPoint(0, 42)});
    QPolygonF qPolygonF = QPolygonF({QPointF(0.5, 0.5), QPointF(4.2, 0.5), QPointF(4.2, 4.2), QPointF(0.5, 4.2)});
    QQuaternion qQuaternion = QQuaternion(0., 0., 0., 1.);
    QRegion qRegion = QRegion();
    QTransform qTransform = QTransform();
    QVector2D qVector2D = QVector2D(42., 42.);
    QVector3D qVector3D = QVector3D(42., 42., 42.);
    QVector4D qVector4D = QVector4D(42., 42., 42., 42.);
};
