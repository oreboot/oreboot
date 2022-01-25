#define A 2
#define B (A+1)
#define MULTILINE_MACRO \
	cmp $A, $B;\
	mov $C, %rsp;\
 pop $rsp; // Multi-line macro ends here. This comment is ignored.
							
