#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "../include/samp_bcrypt.inc"


Test:TestBcryptHash(){
	bcrypt_hash(0,"OnPassswordHash","sreyas",12);
}

forward OnPassswordHash(playerid,hashid);
public OnPassswordHash(playerid,hashid){
	new dest[250];
	bcrypt_get_hash(hashid,dest);
	bcrypt_verify(playerid,"OnPassswordVerify","sreyas",dest);
}

forward OnPassswordVerify(playerid,bool:success);
public OnPassswordVerify(playerid,bool:success){
	ASSERT(success == true);
	print("\nPASS!");
}