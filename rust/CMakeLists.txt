get_target_property(QMAKE Qt6::qmake IMPORTED_LOCATION)

corrosion_import_crate(MANIFEST_PATH Cargo.toml CRATES minesweeper-rs)
corrosion_set_env_vars(minesweeper-rs
    "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
    "QMAKE=${QMAKE}"
)

target_include_directories(minesweeper-rs INTERFACE "${CXXQT_EXPORT_DIR}/minesweeper-rs")
target_link_libraries(minesweeper-rs INTERFACE 
    Qt6::Core 
    Qt6::Gui 
    Qt6::Qml  
    Qt6::Quick  
    Qt6::QuickControls2
    minesweeper-cxx-utils
)

get_target_property(MINESWEEPER_RS_INCLUDES minesweeper-cxx-utils INCLUDE_DIRECTORIES)
foreach(DIRECTORY ${MINESWEEPER_RS_INCLUDES})
    string(RANDOM HASH)
    corrosion_set_env_vars(
        minesweeper-rs
        "MINESWEEPER_RS_INCLUDE_${HASH}=${DIRECTORY}"
    )
endforeach()