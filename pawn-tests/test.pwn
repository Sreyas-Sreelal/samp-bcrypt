#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "../include/samp_bcrypt.inc"

main() { }

Test:RunTest() {
	ASSERT(YourNativeFunctionName() == 1);
}