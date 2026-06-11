#include <stdlib.h>
#include <stdio.h>
#include <getopt.h>
#include "RIFImporter.h"
#include <cwalk.h>
#include <Mime2Rdf4C.h>

const char *datapath;
const char *querypath;
bool expect = true;
const char* format = "ttl";
const char* data_entailment = NULL;

static int parse_args(int argc, char *argv[]);
static char* load_into_memory(const char* filepath);
int check_query(RIFIData *data, RIFIData *query);
RIFIData* load_data(const char* filepath, const char* entailment);

int main(int argc, char *argv[]){
	uint8_t err;
	RIFIData *data, *query;
	err = parse_args(argc, argv);
	if (err != 0){
		exit(EXIT_FAILURE);
	}
	fprintf(stderr, "Create datagraph\n");
	data = load_data(datapath, data_entailment);
	if (data == NULL){
		fprintf(stderr, "Datagraph couldnt be loaded\n");
		exit(EXIT_FAILURE);
	}
	fprintf(stderr, "Create querygraph\n");
	query = load_data(querypath, NULL);
	if (query == NULL){
		fprintf(stderr, "Querygraph couldnt be loaded\n");
		exit(EXIT_FAILURE);
	}

	err = check_query(data, query);
	free_RIFIData(data);
	free_RIFIData(query);
	switch (err){
		case 0:
			if(expect){
				exit(EXIT_SUCCESS);
			} else {
				fprintf(stderr, "Expected failure but "
						"got succes\n");
				exit(EXIT_FAILURE);
			}
		case 1:
			if(expect){
				fprintf(stderr, "Got failure\n");
				exit(EXIT_FAILURE);
			} else {
				exit(EXIT_SUCCESS);
			}
		default:
			fprintf(stderr, "check_query failed to work properly\n");
			exit(EXIT_FAILURE);
	}
}


static struct option parse_options[] = {
	{"data", required_argument, NULL, 'd'},
	{"query", required_argument, NULL, 'q'},
	{"expected-failure", no_argument, NULL, 'x'},
	{"entailment", required_argument, NULL, 'e'},
        {NULL, 0, NULL, 0}
};

static int parse_args(int argc, char *argv[]){
	int err = 0;
	int c = 0;
	int option_index;
	while(c != -1){
		c = getopt_long(argc, argv, "",
				parse_options, &option_index);
		switch(c){
			case -1: //end of arguments
				break;
			case 'x':
				expect = false;
				break;
			case 'd':
				datapath = optarg;
				break;
			case 'q':
				querypath = optarg;
				break;
			case 'e':
				data_entailment = optarg;
				break;

			default:
				fprintf(stderr, "unrecognized argument\n");
				err = 1;
				break;
		}
	}
	return err;
}

static char* load_into_memory(const char* filepath){
        char *ret;
        long fsize;
        FILE *f = fopen(filepath, "rb");
        if (f == NULL) return NULL;
        fseek(f, 0, SEEK_END);
        fsize = ftell(f);
        rewind(f);
        //fseek(f, 0, SEEK_SET);  /* same as rewind(f); */

        ret = malloc(fsize + 1);
        fread(ret, fsize, 1, f);
        ret[fsize] = 0;
        fclose(f);
        return ret;
}


RIFIData* load_data(const char* filepath, const char* entailment){
	int err;
	RIFIData *data;
	char *tmpinput;
	const char* ext;
	Mime2Rdf4C_ParserConfig* config;
	const char* found_ext;
	size_t ext_length;
	if (!cwk_path_get_extension(filepath, &found_ext, &ext_length)){
		fprintf(stderr, "Couldnt find extension to file \"%s\"\n",
				filepath);
		return NULL;
	}
	config = Mime2Rdf4C_get_parser_from_ext(found_ext + 1);
	tmpinput = load_into_memory(filepath);
	if (tmpinput == NULL){
		fprintf(stderr, "Couldnt find data: %s\n", filepath);
		return NULL;
	}
	fprintf(stderr, "input : %s\n", tmpinput);
	data = RIFIData_new(entailment);
	if (data == NULL){
		fprintf(stderr, "Failed to initialize RIFIData\n");
		return NULL;
	}
	err = Mime2Rdf4C_parse(tmpinput, (TripleHandler*) RIFIData_add,
					data, config);
	free_Mime2Rdf4CParserConfig(config);
	free(tmpinput);
	if (err != 0){
		fprintf(stderr, "parsing failed\n");
		return NULL;
	}
	return data;
}
