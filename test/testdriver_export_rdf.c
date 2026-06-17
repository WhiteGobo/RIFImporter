#include <stdlib.h>
#include <stdio.h>
#include <getopt.h>
#include "RIFImporter.h"
#include <Mime2Rdf4C.h>
#include "printhelper.h"

const char *datapath;

static int parse_args(int argc, char *argv[]);
static char* load_into_memory(const char* filepath);
int check_query(RIFIData *data, RIFIData *query);

RIFIData* load_data(const char* filepath);
RIFIData* recreate_data(const char* filepath);


static int8_t myhook(
                const char* subject, uint8_t subject_type,
                const char* predicate,
                const char* object, const char* object_suffix,
                uint8_t object_type,
                const char* graphid, uint8_t graph_type,
                void* user);

int main(int argc, char *argv[]){
	uint8_t err;
	RIFIData *data, *query;
	err = parse_args(argc, argv);
	if (err != 0){
		exit(EXIT_FAILURE);
	}
	data = recreate_data(datapath);
	if (data == NULL){
		fprintf(stderr, "Datagraph couldnt be created\n");
		exit(EXIT_FAILURE);
	}
	query = load_data(datapath);
	if (query == NULL){
		fprintf(stderr, "Querygraph couldnt be loaded\n");
		exit(EXIT_FAILURE);
	}
	err = check_query(data, query);
	free_RIFIData(data);
	free_RIFIData(query);
	switch (err){
		case 0:
			exit(EXIT_SUCCESS);
		default:
			fprintf(stderr, "check_query failed\n");
			exit(EXIT_FAILURE);
	}
}

RIFIData* recreate_data(const char* filepath){
	int err;
	const char* ext = "ttl";
	char* recreated_rdf_data;
	Mime2Rdf4C_ParserConfig* config;
	RIFIData* ret;
	RIFIGraph* ret_data;
	Mime2Rdf4C_SerializerConfig* ttlserializer;
	RIFIData* data = load_data(filepath);
	if (data == NULL){
		fprintf(stderr, "Datagraph couldnt be loaded\n");
		return NULL;
	}
	ttlserializer = Mime2Rdf4C_get_serializer_from_ext(ext);
	if (ttlserializer == NULL) {
		fprintf(stderr, "Failed to initialize serializer\n");
		return NULL;
	}
	/*
	err = RIFIData_send_as_rdf(data,
				(TripleHandler*) Mime2Rdf4C_add,
				ttlserializer);
				*/
	err = RIFIData_send_document_as_rdf(data,
				(TripleHandler*) Mime2Rdf4C_add,
				ttlserializer);

	if (err != 0 ){
		fprintf(stderr, "error during RIFI_send_as_rdf");
		return NULL;
	}
	free_RIFIData(data);
	recreated_rdf_data = Mime2Rdf4C_finish(ttlserializer);
	if (recreated_rdf_data == NULL) {
		fprintf(stderr, "Failed to serialize rif data in rdf\n");
		return NULL;
	}
	fprintf(stderr, "Recreated rif in rdf: %s\n", recreated_rdf_data);
	ret_data = RIFIGraph_new();
	if (data == NULL){
		fprintf(stderr, "Failed to initialize RIFIGraph seconds\n");
		return NULL;
	}
	config = Mime2Rdf4C_get_parser_from_ext(ext);
	err = Mime2Rdf4C_parse(recreated_rdf_data,
				(TripleHandler*) RIFIGraph_add, ret_data,
				config);
	ret = RIFIGraph_to_RIFIData(ret_data, NULL);
	free_Mime2Rdf4CParserConfig(config);
	free(recreated_rdf_data);
	return ret;
}


static struct option parse_options[] = {
	{"data", required_argument, NULL, 'd'},
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
			case 'd':
				datapath = optarg;
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

RIFIData* load_data(const char* filepath){
	int err;
	RIFIData *data;
	RIFIGraph *dataloader;
	char *tmpinput;
	const char* ext = "ttl";
	Mime2Rdf4C_ParserConfig* config;
	config = Mime2Rdf4C_get_parser_from_ext(ext);
	if (config == NULL){
		fprintf(stderr, "couldnt get parser from ext %s\n", ext);
		return NULL;
	}
	tmpinput = load_into_memory(filepath);
	if (tmpinput == NULL){
		fprintf(stderr, "Couldnt find data: %s\n", filepath);
		return NULL;
	}
	fprintf(stderr, "input : %s\n", tmpinput);
	dataloader = RIFIGraph_new();
	if (dataloader == NULL){
		fprintf(stderr, "Failed to initialize RIFIGraph\n");
		return NULL;
	}
	err = Mime2Rdf4C_parse(tmpinput, (TripleHandler*) RIFIGraph_add,
					dataloader, config);
	data = RIFIGraph_to_RIFIData(dataloader, NULL);
	free_Mime2Rdf4CParserConfig(config);
	free(tmpinput);
	if (err != 0){
		fprintf(stderr, "parsing failed\n");
		return NULL;
	}
	return data;
}


static int8_t myhook(
                const char* subject, uint8_t subject_type,
                const char* predicate,
                const char* object, const char* object_suffix,
                uint8_t object_type,
                const char* graphid, uint8_t graph_type,
                void* user){
	fprintf(stderr, "called my hook\n");
	fprintf(stderr, "asdf %s, %s, %s\n", subject, predicate, object);
	return 0;
}
