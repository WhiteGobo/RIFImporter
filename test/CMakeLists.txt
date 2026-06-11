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
add_test(NAME "RIFImporter::export::data02" COMMAND RIFImporter_export_rdf_test
	ARGS "--data" "${basepath}/data02.ttl"
)
set_property(TEST "RIFImporter::export::data02" PROPERTY LABELS "RIFImporter")
add_test(NAME "RIFImporter::export::data03" COMMAND RIFImporter_export_rdf_test
	ARGS "--data" "${basepath}/data03.ttl"
)
set_property(TEST "RIFImporter::export::data03" PROPERTY LABELS "RIFImporter")
add_test(NAME "RIFImporter::export::data04" COMMAND RIFImporter_export_rdf_test
	ARGS "--data" "${basepath}/data04.ttl"
)
set_property(TEST "RIFImporter::export::data04" PROPERTY LABELS "RIFImporter")
add_test(NAME "RIFImporter::export::data05" COMMAND RIFImporter_export_rdf_test
	ARGS "--data" "${basepath}/data05.ttl"
)
set_property(TEST "RIFImporter::export::data05" PROPERTY LABELS "RIFImporter")
add_test(NAME "RIFImporter::export::data06" COMMAND RIFImporter_export_rdf_test
	ARGS "--data" "${basepath}/data06.ttl"
)
set_property(TEST "RIFImporter::export::data06" PROPERTY LABELS "RIFImporter")
add_test(NAME "RIFImporter::export::data07" COMMAND RIFImporter_export_rdf_test
	ARGS "--data" "${basepath}/data07.ttl"
)
set_property(TEST "RIFImporter::export::data07" PROPERTY LABELS "RIFImporter")
add_test(NAME "RIFImporter::export::data08" COMMAND RIFImporter_export_rdf_test
	ARGS "--data" "${basepath}/data08.ttl"
)
set_property(TEST "RIFImporter::export::data08" PROPERTY LABELS "RIFImporter")
add_test(NAME "RIFImporter::export::data09" COMMAND RIFImporter_export_rdf_test
	ARGS "--data" "${basepath}/data09.ttl"
)
set_property(TEST "RIFImporter::export::data09" PROPERTY LABELS "RIFImporter")
