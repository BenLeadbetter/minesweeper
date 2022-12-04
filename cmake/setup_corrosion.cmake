macro(setup_corrosion)
    find_package(Corrosion QUIET)
    if(NOT Corrosion_FOUND)
        include(FetchContent)
        FetchContent_Declare(
            Corrosion
            GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
            GIT_TAG v0.3.0
        )
        FetchContent_MakeAvailable(Corrosion)
    endif()
endmacro()