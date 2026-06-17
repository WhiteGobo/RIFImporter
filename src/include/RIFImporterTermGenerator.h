#pragma once

#include "RIFImporter.h"
#include <stddef.h>

RIFITerm* RIFITerm_new_iri(const char* value);
RIFITerm* RIFITerm_new_typedliteral(const char* value, const char* suffix);
RIFITerm* RIFITerm_new_langliteral(const char* value, const char* suffix);
RIFITerm* RIFITerm_new_list(const RIFITermList* list, const RIFITerm* rest);
RIFITerm* RIFITerm_new_local(const char* value);
RIFIFrame* RIFIFrame_new(RIFITerm* object, RIFITerm* slotkey, RIFITerm* slotvalue);
RIFIAtom* RIFIAtom_new(RIFITerm* op, RIFITermList* args);
RIFISubclass* RIFISubclass_new(RIFITerm* sub, RIFITerm* super);
RIFIMember* RIFIMember_new(RIFITerm* instance, RIFITerm* class);
RIFIEqual* RIFIEqual_new(RIFITerm* left, RIFITerm* right);
