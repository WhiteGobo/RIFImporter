add_library(RIFImporter_CHelperlib
	RIFImporter_CHelperlib.c
)
target_include_directories(RIFImporter_CHelperlib PRIVATE
	"${CMAKE_CURRENT_SOURCE_DIR}/include")

corrosion_import_crate(MANIFEST_PATH ./Cargo.toml)
corrosion_link_libraries(RIFImporter RIFImporter_CHelperlib)
target_sources(RIFImporter INTERFACE
	FILE_SET HEADERS
	BASE_DIRS include
	FILES
	include/RIFImporter.h
	include/RIFImporterTermGenerator.h
	include/RIFImporterDebug.h
)


#corrosion_install(TARGETS CInterfaceOxTTL EXPORT CInterfaceOxTTLtargets)


#target_link_libraries(RIFImporter
#	RIFImporterOxigraphImplementation
#)
