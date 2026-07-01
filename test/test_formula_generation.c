#include <stdlib.h>
#include <stdio.h>
#include <getopt.h>
#include "RIFImporter.h"
#include <Mime2Rdf4C.h>
#include "printhelper.h"

int main(int argc, char *argv[]){
	RIFIFormulas* formulas = RIFIFormulas_new();
	RIFIData* generated_data;
	void RIFIFormulas_add_atom(RIFIFormulas*, RIFIAtom*);
	void RIFIFormulas_add_frame(RIFIFormulas*, RIFIFrame*);
	void RIFIFormulas_add_subclass(RIFIFormulas*, RIFISubclass*);
	void RIFIFormulas_add_member(RIFIFormulas*, RIFIMember*);
	void RIFIFormulas_add_equal(RIFIFormulas*, RIFIEqual*);
	generated_data = RIFIFormulas_to_RIFIData(formulas);
}
