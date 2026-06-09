static void fprintf_term(RIFITerm*);
static void fprintf_list(RIFITermList*);

static void fprintf_term(RIFITerm* term){
	if (term == NULL){
		fprintf(stderr, "?");
		return;
	}
	switch(term->type){
		case RIF_IRI:
			fprintf(stderr, "<%s>", term->value);
			break;
		case RIF_TypedLiteral:
			fprintf(stderr, "\"%s\"^^<%s>", term->value, term->suffix);
			break;
		case RIF_LangLiteral:
			fprintf(stderr, "\"%s\"@%s", term->value, term->suffix);
			break;
		case RIF_List:
			fprintf(stderr, "(");
			fprintf_list(term->list);
			if (term->rest != NULL){
				fprintf(stderr, " | ");
				fprintf_term(term->rest);
			}
			fprintf(stderr, ")");
			break;
		case RIF_Local:
			fprintf(stderr, "local(%s)", term->value);
			break;
	}
}

static void fprintf_list(RIFITermList* list){
	if (list == NULL) return;
	fprintf_term(list->first);
	for (RIFITermList* tmp = list->rest; tmp != NULL; tmp = tmp->rest){
		fprintf(stderr, " ");
		fprintf_term(tmp->first);
	}
}

static void fprintf_atom(RIFIAtom* atom){
	fprintf_term(atom->op);
	fprintf(stderr, "(");
	fprintf_list(atom->args);
	fprintf(stderr, ")");
}

static void fprintf_frame(RIFIFrame* frame){
	fprintf_term(frame->object);
	fprintf(stderr, "[");
	fprintf_term(frame->slotkey);
	fprintf(stderr, " -> ");
	fprintf_term(frame->slotvalue);
	fprintf(stderr, "]");
}

static void fprintf_subclass(RIFISubclass* x){
	fprintf_term(x->sub);
	fprintf(stderr, "##");
	fprintf_term(x->super);
}

static void fprintf_member(RIFIMember* x){
	fprintf_term(x->instance);
	fprintf(stderr, "#");
	fprintf_term(x->class);
}

static void fprintf_equal(RIFIEqual* x){
	fprintf_term(x->left);
	fprintf(stderr, "=");
	fprintf_term(x->right);
}
