include(FetchContent)

FetchContent_Declare(
	Corrosion
	GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
	GIT_TAG v0.6 # Optionally specify a commit hash, version tag or branch here
	FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(Corrosion)


# needed for testing:

FetchContent_Declare(
        cwalk
        GIT_REPOSITORY https://github.com/likle/cwalk.git
	GIT_TAG v1.2.9
	FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(cwalk)

FetchContent_Declare(
	Mime2Rdf4C
	GIT_REPOSITORY https://github.com/WhiteGobo/Mime2Rdf4C.git
	#GIT_TAG 6ae08cf54ec74aa5f8a9d0ca8547155af53be03a
	FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(Mime2Rdf4C)
