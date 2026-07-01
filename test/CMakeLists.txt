include(CTest)

add_executable(RIFImporter_roundtriptest
	testdriver_roundtrip.c
	check_query.c
	reimport_data.c
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

foreach( x IN ITEMS
		"data01"
		"data02"
		"data03"
		"data04"
		"data05"
		"data06"
		"data07"
		"data08"
		"data09"
)
	set(testname "RIFImporter::export::${x}")
	set(datapath "${basepath}/${x}.ttl")
	add_test(NAME "${testname}" COMMAND RIFImporter_export_rdf_test
		ARGS "--data" "${datapath}"
	)
	set_property(TEST ${testname} PROPERTY LABELS "RIFImporter")
endforeach()
