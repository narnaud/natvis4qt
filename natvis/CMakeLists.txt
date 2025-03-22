cmake_minimum_required(VERSION 3.16)

project(test-natvis LANGUAGES CXX)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 REQUIRED COMPONENTS Core Gui Widgets Network)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Core Gui Widgets Network)

qt_add_executable(test-natvis
	test/main.cpp
    test/core_types.h
    test/container_types.h
    test/variant_types.h
    test/network_types.h
    test/gui_types.h
    test/widgets_types.h
)

qt_add_resources(test-natvis "assets"
    PREFIX "/"
    FILES
        test/rustacean.png
        test/pass1.json
)

target_link_libraries(test-natvis
    PRIVATE
        Qt6::Core
        Qt6::Gui
        Qt6::Widgets
        Qt6::Network
)

install(TARGETS test-natvis LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
