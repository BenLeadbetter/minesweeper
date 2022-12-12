add_library(minesweeper-cxx-utils 
    inc/QAbstractTableModelCxx.h 
    inc/QModelIndexCxx.h
)
target_link_libraries(minesweeper-cxx-utils PRIVATE Qt6::Core)
target_include_directories(minesweeper-cxx-utils PUBLIC inc)
target_include_directories(minesweeper-cxx-utils PRIVATE inc)