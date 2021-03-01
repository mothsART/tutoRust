run := cargo run --bin

variables_un:
	$(run) variables_un

variables_deux:
	$(run) variables_deux

variables_trois:
	$(run) variables_trois

variables_quatre:
	$(run) variables_quatre

variables_cinq:
	$(run) variables_cinq

variables_six:
	$(run) variables_six

struct:
	$(run) struct

struct_recursif:
	$(run) struct_recursif

enum:
	$(run) enum

ownership:
	$(run) ownership 

array:
	$(run) array

vec:
	$(run) vec

slice:
	$(run) slice

pattern_matching_un:
	$(run) pattern_matching_un

pattern_matching_deux:
	$(run) pattern_matching_deux

map_filter_closure:
	$(run) map_filter_closure

implement:
	$(run) implement

trait:
	$(run) trait
