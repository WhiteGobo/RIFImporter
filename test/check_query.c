#include <stdlib.h>
#include <stdio.h>
#include <getopt.h>
#include "RIFImporter.h"
#include <cwalk.h>
#include <Mime2Rdf4C.h>
#include "printhelper.h"

static void replace_frame_blanks(RIFIFrame*);
static void replace_atom_blanks(RIFIAtom*);
static void replace_subclass_blanks(RIFISubclass*);
static void replace_member_blanks(RIFIMember*);
static void replace_equal_blanks(RIFIEqual*);

int check_query(RIFIData *data, RIFIData *query){
	RIFIFrame* frame;
	RIFIAtom* atom;
	RIFIEqual* equal;
	RIFISubclass* subclass;
	RIFIMember* member;
	fprintf(stderr, "check query\n");
	for(RIFIAtom* q = RIFIData_get_next_atom_any_args(query, NULL);
			q != NULL;
			q = RIFIData_get_next_atom_any_args(query, NULL))
	{
		fprintf(stderr, "query atom: ");
		fprintf_atom(q);
		fprintf(stderr, "\n");
		replace_atom_blanks(q);
		atom = RIFIData_get_next_atom(data, q->op, q->args);
		free_RIFIAtom(q);
		if (atom == NULL){
			fprintf(stderr, "Failed to find atom\n");
			return 1;
		}
		fprintf(stderr, "found: ");
		fprintf_atom(atom);
		fprintf(stderr, "\n");
		free_RIFIAtom(atom);
	}
	for(RIFIFrame* q = RIFIData_get_next_frame(query, NULL, NULL, NULL);
			q != NULL;
			q = RIFIData_get_next_frame(query, NULL, NULL, NULL))
	{
		replace_frame_blanks(q);
		fprintf(stderr, "query frame: ");
		fprintf_frame(q);
		fprintf(stderr, "\n");
		frame = RIFIData_get_next_frame(data, q->object,
						q->slotkey, q->slotvalue);
		free_RIFIFrame(q);
		if (frame == NULL){
			fprintf(stderr, "Failed to find frame\n");
			return 1;
		}
		fprintf(stderr, "found frame: ");
		fprintf_frame(frame);
		fprintf(stderr, "\n");
		free_RIFIFrame(frame);
	}
	for(RIFISubclass* q = RIFIData_get_next_subclass(query, NULL, NULL);
			q != NULL;
			q = RIFIData_get_next_subclass(query, NULL, NULL))
	{
		fprintf(stderr, "query subclass: ");
		fprintf_subclass(q);
		fprintf(stderr, "\n");
		replace_subclass_blanks(q);
		subclass = RIFIData_get_next_subclass(data, q->sub, q->super);
		free_RIFISubclass(q);
		if (subclass == NULL){
			fprintf(stderr, "Failed to find subclass\n");
			return 1;
		}
		free_RIFISubclass(subclass);
	}
	for(RIFIMember* q = RIFIData_get_next_member(query, NULL, NULL);
			q != NULL;
			q = RIFIData_get_next_member(query, NULL, NULL))
	{
		fprintf(stderr, "query member: ");
		fprintf_member(q);
		fprintf(stderr, "\n");
		replace_member_blanks(q);
		member = RIFIData_get_next_member(data, q->instance, q->class);
		free_RIFIMember(q);
		if (member == NULL){
			fprintf(stderr, "Failed to find member\n");
			return 1;
		}
		free_RIFIMember(member);
	}
	for(RIFIEqual* q = RIFIData_get_next_equal(query, NULL, NULL);
			q != NULL;
			q = RIFIData_get_next_equal(query, NULL, NULL))
	{
		fprintf(stderr, "query equal: ");
		fprintf_equal(q);
		fprintf(stderr, "\n");
		replace_equal_blanks(q);
		equal = RIFIData_get_next_equal(data, q->left, q->right);
		free_RIFIEqual(q);
		if (equal == NULL){
			fprintf(stderr, "Failed to find equal\n");
			return 1;
		}
		free_RIFIEqual(equal);
	}
	return 0;
}



static void replace_frame_blanks(RIFIFrame*){
}
static void replace_atom_blanks(RIFIAtom*){
}
static void replace_subclass_blanks(RIFISubclass*){
}
static void replace_member_blanks(RIFIMember*){
}
static void replace_equal_blanks(RIFIEqual*){
}
