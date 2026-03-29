cmake_minimum_required(VERSION 3.16)

set(EXTRA_COMPONENT_DIRS
    "${CMAKE_CURRENT_LIST_DIR}/../xiaozhi-esp32/components/i2c_bsp"
    "${CMAKE_CURRENT_LIST_DIR}/../xiaozhi-esp32/components/axpPower"
    "${CMAKE_CURRENT_LIST_DIR}/../xiaozhi-esp32/components/ListLib"
    "${CMAKE_CURRENT_LIST_DIR}/../xiaozhi-esp32/components/sdcard_bsp"
    "${CMAKE_CURRENT_LIST_DIR}/../xiaozhi-esp32/components/button_bsp"
    "${CMAKE_CURRENT_LIST_DIR}/../xiaozhi-esp32/components/epaper_port"
    "${CMAKE_CURRENT_LIST_DIR}/../xiaozhi-esp32/components/epaper_src")

include($ENV{IDF_PATH}/tools/cmake/project.cmake)
project(firmware_http_epaper)
