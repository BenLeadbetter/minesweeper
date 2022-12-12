use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .cc_builder(|cc| {
            cc.includes(
                std::env::vars()
                    .filter(|(key, _)| key.contains("MINESWEEPER_RS_INCLUDE"))
                    .map(|(_, val)| val)
            );
        })
        .file("src/minefield.rs")
        .build();
}