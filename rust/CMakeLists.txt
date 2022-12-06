get_target_property(QMAKE Qt6::qmake IMPORTED_LOCATION)

corrosion_import_crate(MANIFEST_PATH Cargo.toml CRATES minesweeper-rs)
corrosion_set_env_vars(minesweeper-rs
    "CXXQT_EXPORT_DIR=${CXXQT_EXPORT_DIR}"
    "QMAKE=${QMAKE}"
)

target_include_directories(minesweeper-rs INTERFACE "${CXXQT_EXPORT_DIR}/minesweeper-rs")
