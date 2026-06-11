include(CTest)

add_executable(RIFImporter_roundtriptest
	testdriver_roundtrip.c
	check_query.c
	printhelper.h
)

target_link_libraries(RIFImporter_roundtriptest
	RIFImporter
	Mime2Rdf4C
	cwalk
)

add_executable(RIFImporter_export_rdf_test
	testdriver_export_rdf.c
	check_query.c
	printhelper.h
)

target_link_libraries(RIFImporter_export_rdf_test
	RIFImporter
	Mime2Rdf4C
)

include(tools.cmake)

include(configure_roundtrip.cmake)


set(basepath "${CMAKE_CURRENT_SOURCE_DIR}/data_export")

add_test(NAME "RIFImporter::export::data01" COMMAND RIFImporter_export_rdf_test
	ARGS "--data" "${basepath}/data01.ttl"
)
set_property(TEST "RIFImporter::export::data01" PROPERTY LABELS "RIFImporter")
