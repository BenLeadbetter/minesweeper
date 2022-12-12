cmake_minimum_required(VERSION 3.25)

project(minesweeper VERSION 0.1)
set(APP_NAME ${PROJECT_NAME})

set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")
include(project_init)

setup_conan()
setup_corrosion()

conan_cmake_configure(
    REQUIRES qt/6.3.2
    GENERATORS cmake_find_package
    OPTIONS 
        qt:qtdeclarative=True
        qt:qtshadertools=True
        qt:shared=True
)
conan_cmake_autodetect(CONAN_SETTINGS)
conan_cmake_install(
    PATH_OR_REFERENCE .
    BUILD missing
    REMOTE conancenter
    SETTINGS ${CONAN_SETTINGS}
)

find_package(Qt6 COMPONENTS Core Gui Qml Quick QuickControls2 REQUIRED)

add_subdirectory(cpp)
add_subdirectory(rust)
add_subdirectory(qml)

add_executable(${APP_NAME} main.cpp)
target_link_libraries(
    ${APP_NAME} 
        PRIVATE 
            minesweeper-rs 
            minesweeper-qml-resources
)

set_target_properties(${APP_NAME} PROPERTIES
    MACOSX_BUNDLE_GUI_IDENTIFIER ${APP_NAME}.com
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
)
