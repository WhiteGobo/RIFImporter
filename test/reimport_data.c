#include <stdlib.h>
#include <stdio.h>
#include "RIFImporter.h"
#include "RIFImporterTermGenerator.h"

RIFIData* reimport_data(RIFIData* inputdata){
	RIFIFrame* frame;
	RIFIAtom* atom;
	RIFIEqual* equal;
	RIFISubclass* subclass;
	RIFIMember* member;
	RIFIFormulas* reimported_formulas = RIFIFormulas_new();
	if(reimported_formulas == NULL){
		fprintf(stderr, "RIFIFormulas_new failed\n");
		return NULL;
	}
	for(RIFIAtom* q = RIFIData_get_next_atom_any_args(inputdata, NULL);
			q != NULL;
			q = RIFIData_get_next_atom_any_args(inputdata, NULL))
	{
		RIFIFormulas_add_atom(reimported_formulas, q);
		free_RIFIAtom(q);
	}
	for(RIFIFrame* q = RIFIData_get_next_frame(inputdata, NULL, NULL, NULL);
			q != NULL;
			q = RIFIData_get_next_frame(inputdata, NULL, NULL, NULL))
	{
		RIFIFormulas_add_frame(reimported_formulas, q);
		free_RIFIFrame(q);
	}
	for(RIFISubclass* q = RIFIData_get_next_subclass(inputdata, NULL, NULL);
			q != NULL;
			q = RIFIData_get_next_subclass(inputdata, NULL, NULL))
	{
		RIFIFormulas_add_subclass(reimported_formulas, q);
		free_RIFISubclass(q);
	}
	for(RIFIMember* q = RIFIData_get_next_member(inputdata, NULL, NULL);
			q != NULL;
			q = RIFIData_get_next_member(inputdata, NULL, NULL))
	{
		RIFIFormulas_add_member(reimported_formulas, q);
		free_RIFIMember(q);
	}
	for(RIFIEqual* q = RIFIData_get_next_equal(inputdata, NULL, NULL);
			q != NULL;
			q = RIFIData_get_next_equal(inputdata, NULL, NULL))
	{
		RIFIFormulas_add_equal(reimported_formulas, q);
		free_RIFIEqual(q);
	}
	return RIFIFormulas_to_RIFIData(reimported_formulas);
}
