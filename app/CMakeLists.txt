add_executable(minesweeper main.cpp)
target_link_libraries(
    minesweeper 
        PRIVATE 
            minesweeper-rs 
            minesweeper-qml-resources
)

set_target_properties(minesweeper PROPERTIES
    MACOSX_BUNDLE_GUI_IDENTIFIER minesweeper.com
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
)