cmake_minimum_required(VERSION 3.25)

project(minesweeper)
set(APP_NAME ${PROJECT_NAME})

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")
include(project_init)

setup_conan()
setup_corrosion()

conan_cmake_configure(
    REQUIRES qt/6.3.1 
    GENERATORS cmake_find_package
    OPTIONS qt:qtshadertools=True
    OPTIONS qt:qtdeclarative=True
)
conan_cmake_autodetect(CONAN_SETTINGS)
conan_cmake_install(
    PATH_OR_REFERENCE .
    BUILD missing
    REMOTE conancenter
    SETTINGS ${CONAN_SETTINGS}
)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 COMPONENTS Core Gui Qml QuickControls2 REQUIRED)
if(QT_STATIC)
    find_package(Qt6 COMPONENTS QmlImportScanner REQUIRED)
endif()
get_target_property(QMAKE Qt6::qmake IMPORTED_LOCATION)

set(CRATE minesweeper-rs)
corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml CRATES ${CRATE})

set(CXXQT_EXPORT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cxxqt")
corrosion_set_env_vars(${CRATE}
    "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
    "QMAKE=${QMAKE}"
)

target_include_directories(${CRATE} INTERFACE "${CXXQT_EXPORT_DIR}/${CRATE}")
target_link_libraries(${CRATE} INTERFACE 
    Qt6::Core 
    Qt6::Gui 
    Qt6::Qml 
    Qt6::QuickControls2 
)
add_executable(${APP_NAME} cpp/main.cpp qml/qml.qrc)
target_link_libraries(${APP_NAME} PRIVATE ${CRATE})

if(QT_STATIC)
    qt_import_qml_plugins(${APP_NAME})
endif()