#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "../include/samp_bcrypt.inc"

Test:TestBcryptSetNumThreads(){
	ASSERT(bcrypt_set_thread_limit(-1) == 0);
	ASSERT(bcrypt_set_thread_limit(0) == 0);
	ASSERT(bcrypt_set_thread_limit(3) == 1);
}

Test:TestBcryptHash(){
	bcrypt_hash(0,"OnPassswordHash","text",12);
	bcrypt_hash(0,"OnPassswordHash2","test",4);
}

forward OnPassswordHash(playerid);
public OnPassswordHash(playerid){
	printf("***OnPassswordHash");
	new dest[250];
	bcrypt_get_hash(dest);
	printf("hash is %s",dest);
	bcrypt_verify(playerid,"OnPassswordVerifyValid","text",dest);
	bcrypt_verify(playerid,"OnPassswordVerifyInvalid","test",dest);
}

forward OnPassswordHash2(playerid);
public OnPassswordHash2(playerid){
	printf("***OnPassswordHash2");
	new dest[250];
	bcrypt_get_hash(dest);
	printf("hash is %s",dest);
	bcrypt_verify(playerid,"OnPassswordVerifyInvalid","text",dest);
	bcrypt_verify(playerid,"OnPassswordVerifyValid","test",dest);
}
forward OnPassswordVerifyValid(playerid,bool:success);
public OnPassswordVerifyValid(playerid,bool:success){
	printf("***OnPassswordVerifyValid");
	ASSERT(success == true);
	print("\nPASS!");
}

forward OnPassswordVerifyInvalid(playerid,bool:success);
public OnPassswordVerifyInvalid(playerid,bool:success){
	printf("***OnPassswordVerifyInvalid");
	ASSERT(success == false);
	print("\nPASS!");
}