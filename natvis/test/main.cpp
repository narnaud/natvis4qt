#include "container_types.h"
#include "core_types.h"
#include "gui_types.h"
#include "network_types.h"
#include "variant_types.h"
#include "widgets_types.h"

#include <QtWidgets>

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);

    auto coreTypes = CoreTypes();

    auto containerTypes = ContainerTypes();

    auto variantTypes = VariantTypes();

    auto networkTypes = NetworkTypes();

    auto guiTypes = GuiTypes();
    // No ideas why it doesn't work inside guiTypes

    auto qui_QBitmap = QBitmap(":/test/qt-logo.png");
    auto qui_QImage = QImage(":/test/qt-logo.png");
    auto qui_QPixmap = QPixmap(":/test/qt-logo.png");

    auto widgetTypes = WidgetsTypes();

    return 0;
}
