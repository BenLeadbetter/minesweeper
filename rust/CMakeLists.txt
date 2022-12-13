get_target_property(QMAKE Qt6::qmake IMPORTED_LOCATION)

corrosion_import_crate(MANIFEST_PATH Cargo.toml CRATES minesweeper-rs)
corrosion_set_env_vars(minesweeper-rs
    "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
    "MINESWEEPER_RS_INCLUDE_1=${CMAKE_CURRENT_LIST_DIR}/inc"
    "QMAKE=${QMAKE}"
)

target_include_directories(minesweeper-rs INTERFACE "${CXXQT_EXPORT_DIR}/minesweeper-rs")
target_include_directories(minesweeper-rs INTERFACE inc)
target_link_libraries(minesweeper-rs INTERFACE 
    Qt6::Core 
    Qt6::Gui 
    Qt6::Qml  
    Qt6::Quick  
    Qt6::QuickControls2
)