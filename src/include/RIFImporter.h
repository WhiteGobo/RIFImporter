#pragma once

#include <stdint.h>

#define _RIFENTAIL_ "http://www.w3.org/ns/entailment/"
#define _RIFENTAIL_SIMPLE_ _RIFENTAIL_ "Simple"
#define _RIFENTAIL_RDF_ _RIFENTAIL_ "RDF"
#define _RIFENTAIL_RDFS_ _RIFENTAIL_ "RDFS"
#define _RIFENTAIL_D_ _RIFENTAIL_ "D"
#define _RIFENTAIL_OWLRDFBASED_ _RIFENTAIL_ "OWL-RDF-Based"
#define _RIFENTAIL_RIF_ _RIFENTAIL_ "RIF"
#define _RIFENTAIL_OWLDIRECT_ _RIFENTAIL_ "OWL-Direct"

#ifndef RDF_TERMTYPE_DEFINED
#define RDF_TERMTYPE_DEFINED
typedef enum {
        URI = 0,
        BNODE = 1,
        TYPEDLITERAL = 2,
        LANGLITERAL = 3
} TERMTYPE;
#endif

#ifndef TRIPLEHANDLER_DEFINED
#define TRIPLEHANDLER_DEFINED
/*
 * Use TERMTYPE for subject_type, object_type and graph_type.
 * If graphid is NULL, the default graph is used.
 */
typedef int8_t TripleHandler(
                const char* subject, uint8_t subject_type,
                const char* predicate,
                const char* object, const char* object_suffix,
                uint8_t object_type,
                const char* graphid, uint8_t graph_type,
                void* user);

#endif //TRIPLEHANDLER_DEFINED

typedef enum {
	RIF_IRI = 0,
	RIF_TypedLiteral = 1,
	RIF_LangLiteral = 2,
	RIF_List = 3,
	RIF_Local = 4,
} RIFITermType;

typedef struct rifiGraph RIFIGraph;
typedef struct rifiData RIFIData;
typedef struct rifiTermList RIFITermList;
typedef struct rifiTerm {
	uint8_t type;
	union {
		char *value;
		RIFITermList* list;
	};
	union {
		char *suffix;
		struct rifiTerm* rest;
	};
} RIFITerm;

typedef struct rifiTermList {
	RIFITerm* first;
	struct rifiTermList* rest;
} RIFITermList;

typedef struct rifiFrame {
	RIFITerm* object;
	RIFITerm* slotkey;
	RIFITerm* slotvalue;
} RIFIFrame;

typedef struct rifiAtom {
	RIFITerm* op;
	RIFITermList* args;
} RIFIAtom;

typedef struct rifiSubclass {
	RIFITerm* sub;
	RIFITerm* super;
} RIFISubclass;

typedef struct rifiMember {
	RIFITerm* instance;
	RIFITerm* class;
} RIFIMember;

typedef struct rifiEqual {
	RIFITerm* left;
	RIFITerm* right;
} RIFIEqual;


RIFIGraph* RIFIGraph_new();
RIFIData* RIFIGraph_to_RIFIData(RIFIGraph*, const char* entailment);
int8_t RIFIGraph_add(
                const char* subject, uint8_t subject_type,
                const char* predicate,
                const char* object, const char* object_suffix,
                uint8_t object_type,
                const char* graphid, uint8_t graph_type,
                RIFIGraph* user);

int64_t RIFIData_send_as_rdf(RIFIData*, TripleHandler* hook, void* hook_data);
int64_t RIFIData_send_document_as_rdf(RIFIData*, TripleHandler* hook, void* hook_data);

uint64_t RIFIData_remaining(RIFIData*);
void free_RIFIData(RIFIData*);

RIFIAtom* RIFIData_get_next_atom_any_args(RIFIData*, RIFITerm* op);
RIFIAtom* RIFIData_get_next_atom(RIFIData*, RIFITerm* op, RIFITermList* args);
RIFIFrame* RIFIData_get_next_frame(RIFIData*, RIFITerm* object, RIFITerm* slotkey, RIFITerm* slotvalue);
RIFISubclass* RIFIData_get_next_subclass(RIFIData*, RIFITerm* sub, RIFITerm* super);
RIFIMember* RIFIData_get_next_member(RIFIData*, RIFITerm* instance, RIFITerm* class);
RIFIEqual* RIFIData_get_next_equal(RIFIData*, RIFITerm* left, RIFITerm* right);

RIFITermList* RIFITermList_append(RIFITermList*, RIFITerm*);

void free_RIFIAtom(RIFIAtom*);
void free_RIFIFrame(RIFIFrame*);
void free_RIFISubclass(RIFISubclass*);
void free_RIFIEqual(RIFIEqual*);
void free_RIFIMember(RIFIMember*);
void free_RIFITerm(RIFITerm*);
void free_RIFITermList(RIFITermList*);
