#define MACRO \
	(1)// Test test test

#include <test.inc>

/**
block
comment
*/

#define C (1 << 10) // EOL comment

test:
    mov $MACRO, %rsp /* test */
    mov $A, %rsp
    mov $B, %rsp
    mov $C, %rsp
				$MULTILINE_MACRO
