#define RUN_TESTS

#include <a_samp>
#include <YSI_Core\y_testing>

#include "../include/samp_bcrypt.inc"

Test:TestBcryptSetNumThreads(){
	ASSERT(bcrypt_set_thread_limit(-1) == 0);
	ASSERT(bcrypt_set_thread_limit(0) == 0);
	ASSERT(bcrypt_set_thread_limit(3) == 1);
}

Test:TestBcryptHash(){
	bcrypt_hash(0,"OnPassswordHash","text",12);
	bcrypt_hash(0,"OnPassswordHash2","test",4);
	bcrypt_hash(0,"OnPassswordHash3","test",4,"issf",69,"hello","world",10.0);
	
}

Test:TestInvalidCustomArgs() {
	ASSERT(bcrypt_hash(0,"WontCall","test",4,"issf",69,"world",10.0)==0);
	ASSERT(bcrypt_hash(0,"WontCall","test",4,"issf",69,"world",10.0,1,1)==0);
	ASSERT(bcrypt_hash(0,"WontCall","test",4,"issf",69,"world",10.0,1,1)==0);

	ASSERT(bcrypt_verify(0,"WontCall","test","test_hash","issf",69,"world",10.0)==0);
	ASSERT(bcrypt_verify(0,"WontCall","test","test_hash","issf",69,"world",10.0,1,1)==0);
	ASSERT(bcrypt_verify(0,"WontCall","test","test_hash","issf",69,"world",10.0,1,1)==0);
}

forward OnPassswordHash(playerid);
public OnPassswordHash(playerid){
	printf("***OnPassswordHash");
	new dest[BCRYPT_HASH_LENGTH];
	bcrypt_get_hash(dest);
	printf("hash is %s",dest);
	bcrypt_verify(playerid,"OnPassswordVerifyValid","text",dest);
	bcrypt_verify(playerid,"OnPassswordVerifyInvalid","test",dest);
}

forward OnPassswordHash3(playerid,int1,str1[],str2[],Float:float1);
public OnPassswordHash3(playerid,int1,str1[],str2[],Float:float1){
	printf("***OnPassswordHash3");
	ASSERT(int1 == 69);
	new comp1 = strcmp("hello",str1);
	new comp2 = strcmp("world",str2);
	
	ASSERT(int1 == 69);
	ASSERT(comp1 == 0);
	ASSERT(comp2 == 0);
	ASSERT(float1 == 10.0);
	new dest[250];
	bcrypt_get_hash(dest);
	printf("hash is %s",dest);
	bcrypt_verify(playerid,"OnPassswordVerifyInvalid","text",dest);
	bcrypt_verify(playerid,"OnPassswordVerifyValid","test",dest);
	bcrypt_verify(playerid,"OnPassswordVerifyValidWithArgs","test",dest,"issf",69,"hello","world",10.0);
}

forward OnPassswordHash2(playerid);
public OnPassswordHash2(playerid){
	printf("***OnPassswordHash2");
	new dest[BCRYPT_HASH_LENGTH];
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

forward OnPassswordVerifyValidWithArgs(playerid,bool:success,int1,str1[],str2[],Float:float1);
public OnPassswordVerifyValidWithArgs(playerid,bool:success,int1,str1[],str2[],Float:float1){
	printf("***OnPassswordVerifyValidWithArgs");
	ASSERT(success == true);
	ASSERT(int1 == 69);
	new comp1 = strcmp("hello",str1);
	new comp2 = strcmp("world",str2);
	
	ASSERT(int1 == 69);
	ASSERT(comp1 == 0);
	ASSERT(comp2 == 0);
	ASSERT(float1 == 10.0);
	print("\nPASS!");
}

forward OnPassswordVerifyInvalid(playerid,bool:success);
public OnPassswordVerifyInvalid(playerid,bool:success){
	printf("***OnPassswordVerifyInvalid");
	ASSERT(success == false);
	print("\nPASS!");
}
