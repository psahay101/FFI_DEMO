cmake_minimum_required(VERSION 3.0)
project(LibFfiDemo C)
add_library(ffidemo STATIC ffidemo.c)
install(TARGETS ffidemo DESTINATION .)
