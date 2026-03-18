cmake_minimum_required(VERSION 3.23)

project(DoorstopProject)

add_library(Doorstop)

set_property(TARGET Doorstop PROPERTY CXX_STANDARD 20)

add_compile_definitions(UNICODE)

target_sources(Doorstop
	PRIVATE
		dllmain.cpp
		
	PUBLIC
		FILE_SET HEADERS
		FILES
			dllmain.h
)