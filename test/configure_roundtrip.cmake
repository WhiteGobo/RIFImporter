

set(basepath "${CMAKE_CURRENT_SOURCE_DIR}/data")
read_manifest("${basepath}/manifest.json" testarray length)

set(basename "RIFImporter::roundtrip::")
math(EXPR range "${length} - 1")
foreach(x RANGE 0 ${range})
	set(extras "")
	string(JSON data GET ${testarray} ${x})
	string(JSON testsuffix GET ${data} name)
	string(JSON premise GET ${data} premise)
	string(JSON query GET ${data} query)
	string(JSON testtype GET ${data} "@type")
	if (testtype MATCHES "NegativeEntailment")
		list(APPEND extras "--expected-failure")
	endif()
	string(JSON entailment ERROR_VARIABLE ignoreError
		GET ${data} entailment)
	if(NOT ${entailment} MATCHES "NOTFOUND")
		string(JSON ent_length LENGTH ${entailment})
		math(EXPR ent_range "${ent_length} - 1")
		foreach(y RANGE 0 ${ent_range})
			set(tmp_extras "${extras}")
			string(JSON tmp_ent GET ${entailment} ${y})
			list(APPEND tmp_extras "--entailment")
			list(APPEND tmp_extras "${tmp_ent}")
			# cut of "http://www.w3.org/ns/entailment/"
			string(SUBSTRING "${tmp_ent}" 32 -1 tmp_ent_name)
			set(testname "${basename}${testsuffix}::${tmp_ent_name}")
			add_test(NAME "${testname}" COMMAND RIFImporter_roundtriptest
				ARGS
				"--data" "${basepath}/${premise}"
				"--query" "${basepath}/${query}"
				${tmp_extras}
			)
			set_property(TEST "${testname}" PROPERTY LABELS "RIFImporter" "${testtype}")
		endforeach()
	else()
		set(testname "${basename}${testsuffix}")
		add_test(NAME "${testname}" COMMAND RIFImporter_roundtriptest
			ARGS
			"--data" "${basepath}/${premise}"
			"--query" "${basepath}/${query}"
			${extras}
		)
		set_property(TEST "${testname}" PROPERTY LABELS "RIFImporter" "${testtype}")
	endif()


endforeach()

foreach( x IN ITEMS
)
	set_property(TEST "${x}" PROPERTY WILL_FAIL TRUE)
	get_property(tmp_labels TEST "${x}" PROPERTY LABELS)
	list(APPEND tmp_labels "ExpectError")
	set_property(TEST "${x}" PROPERTY LABELS "${tmp_labels}")
endforeach()
