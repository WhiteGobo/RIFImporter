#pragma once 

#include "RIFImporter.h"

void fprintf_RIFITerm(FILE*, RIFITerm*);
void fprintf_RIFITermList(FILE*, RIFITermList*);
void fprintf_RIFIAtom(FILE*, RIFIAtom* atom);
void fprintf_RIFIFrame(FILE*, RIFIFrame* frame);
void fprintf_RIFISubclass(FILE*, RIFISubclass* x);
void fprintf_RIFIMember(FILE*, RIFIMember* x);
void fprintf_RIFIEqual(FILE*, RIFIEqual* x);
