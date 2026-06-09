#include "RIFImporter.h"
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

char* copy2cstring(const char* input, uint64_t size);
RIFITermList* RIFITermList_clone(const RIFITermList* list);
RIFITerm* RIFITerm_clone(const RIFITerm* term);




void free_RIFIAtom(RIFIAtom* atom)
{
	if (atom == NULL) return;
	free_RIFITerm(atom->op);
	free_RIFITermList(atom->args);
	free(atom);
}

void free_RIFIFrame(RIFIFrame* frame)
{
	if (frame == NULL) return;
	free_RIFITerm(frame->object);
	free_RIFITerm(frame->slotkey);
	free_RIFITerm(frame->slotvalue);
	free(frame);
}

void free_RIFISubclass(RIFISubclass* subclass)
{
	if (subclass == NULL) return;
	free_RIFITerm(subclass->sub);
	free_RIFITerm(subclass->super);
	free(subclass);
}

void free_RIFIEqual(RIFIEqual* equal)
{
	if (equal == NULL) return;
	free_RIFITerm(equal->left);
	free_RIFITerm(equal->right);
	free(equal);
}

void free_RIFIMember(RIFIMember* member)
{
	if (member == NULL) return;
	free_RIFITerm(member->instance);
	free_RIFITerm(member->class);
	free(member);
}

void free_RIFITerm(RIFITerm* term)
{
	if (term == NULL) return;
	switch (term->type){
		case RIF_TypedLiteral:
		case RIF_LangLiteral:
			free(term->suffix);
		case RIF_IRI:
		case RIF_Local:
			free(term->value);
			free(term);
			break;
		case RIF_List:
			free_RIFITerm(term->rest);
			free_RIFITermList(term->list);
			free(term);
			break;
	}
}

void free_RIFITermList(RIFITermList* x)
{
	if (x == NULL) return;
	RIFITermList *rest;
	while (x != NULL){
		rest = x->rest;
		free_RIFITerm(x->first);
		x = rest;
	}
}

RIFITerm* RIFITerm_new_iri(const char* value, uint64_t value_length){
	RIFITerm* ret = malloc(sizeof(RIFITerm));
	if (ret == NULL) return NULL;
	ret->type = RIF_IRI;
	ret->value = copy2cstring(value, value_length);
	ret->suffix = NULL;
	return ret;
}
RIFITerm* RIFITerm_new_typedliteral(const char* value, uint64_t value_length, const char* suffix, uint64_t suffix_length){
	RIFITerm* ret = malloc(sizeof(RIFITerm));
	if (ret == NULL) return NULL;
	ret->type = RIF_TypedLiteral;
	ret->value = copy2cstring(value, value_length);
	ret->suffix = copy2cstring(suffix, suffix_length);
	return ret;
}

RIFITerm* RIFITerm_new_langliteral(const char* value, uint64_t value_length, const char* suffix, uint64_t suffix_length)
{
	RIFITerm* ret = malloc(sizeof(RIFITerm));
	if (ret == NULL) return NULL;
	ret->type = RIF_LangLiteral;
	ret->value = copy2cstring(value, value_length);
	ret->suffix = copy2cstring(suffix, suffix_length);
	return ret;
}

RIFITerm* RIFITerm_new_list(const RIFITermList* list, const RIFITerm* rest){
	RIFITerm* ret = malloc(sizeof(RIFITerm));
	if (ret == NULL) return NULL;
	ret->type = RIF_List;
	ret->list = RIFITermList_clone(list);
	ret->rest = RIFITerm_clone(rest);
	return ret;
}

RIFITerm* RIFITerm_new_local(const char* value, uint64_t value_length){
	RIFITerm* ret = malloc(sizeof(RIFITerm));
	if (ret == NULL) return NULL;
	ret->type = RIF_Local;
	ret->value = copy2cstring(value, value_length);
	ret->suffix = NULL;
	return ret;
}

char* copy2cstring(const char* input, uint64_t size){
	if (input == NULL) return NULL;
	char* ret = malloc(size + 1);
	memcpy(ret, input, size);
	ret[size] = '\0';
	return ret;
}

RIFITermList* RIFITermList_clone(const RIFITermList* list){
	RIFITermList* ret = NULL;
	const RIFITermList* tmp = list;
	if (list == NULL) return NULL;
	while (tmp != NULL){
		ret = RIFITermList_append(ret, RIFITerm_clone(tmp->first));
		tmp = tmp->rest;
	}
	return ret;
}


RIFITermList* RIFITermList_append(RIFITermList* old, RIFITerm* term){
	RIFITermList* tmp;
	RIFITermList* new = malloc(sizeof(RIFITermList));
	if (new == NULL){
		fprintf(stderr, "allocation error\n");
		return NULL;
	}
	new->first = RIFITerm_clone(term);
	new->rest = NULL;
	if (old == NULL){
		return new;
	}
	tmp = old;
	while(tmp->rest != NULL){
		tmp = tmp->rest;
	}
	tmp->rest = new;
	return old;
}

RIFITerm* RIFITerm_clone(const RIFITerm* term){
	uint64_t slen, vlen;
	if (term == NULL) return NULL;
	switch (term->type){
		case RIF_IRI:
			return RIFITerm_new_iri(term->value, strlen(term->value));
		case RIF_TypedLiteral:
			vlen = strlen(term->value);
			if (term->suffix != NULL) {
				slen = strlen(term->suffix);
			} else {
				slen = 0;
			}
			return RIFITerm_new_typedliteral(term->value, vlen,
							term->suffix, slen);
		case RIF_LangLiteral:
			vlen = strlen(term->value);
			slen = strlen(term->suffix);
			return RIFITerm_new_langliteral(term->value, vlen,
							term->suffix, slen);
		case RIF_List:
			return RIFITerm_new_list(term->list, term->rest);
		case RIF_Local:
			vlen = strlen(term->value);
			return RIFITerm_new_local(term->value, vlen);
		default:
			return NULL;
	}
}


RIFIFrame* RIFIFrame_new(RIFITerm* object, RIFITerm* slotkey, RIFITerm* slotvalue)
{
	RIFIFrame* ret = malloc(sizeof(RIFIFrame));
	if (ret == NULL) return NULL;
	ret->object = object;
	ret->slotkey = slotkey;
	ret->slotvalue = slotvalue;
	return ret;
}


RIFIAtom* RIFIAtom_new(RIFITerm* op, RIFITermList* args){
	RIFIAtom* ret = malloc(sizeof(RIFIAtom));
	if (ret == NULL) return NULL;
	ret->op = op;
	ret->args = args;
	return ret;
}


RIFISubclass* RIFISubclass_new(RIFITerm* sub, RIFITerm* super){
	RIFISubclass* ret = malloc(sizeof(RIFISubclass));
	if (ret == NULL) return NULL;
	ret->sub = sub;
	ret->super = super;
	return ret;
}


RIFIMember* RIFIMember_new(RIFITerm* instance, RIFITerm* class){
	RIFIMember* ret = malloc(sizeof(RIFIMember));
	if (ret == NULL) return NULL;
	ret->instance = instance;
	ret->class = class;
	return ret;
}


RIFIEqual* RIFIEqual_new(RIFITerm* left, RIFITerm* right){
	RIFIEqual* ret = malloc(sizeof(RIFIEqual));
	if (ret == NULL) return NULL;
	ret->left = left;
	ret->right = right;
	return ret;
}
