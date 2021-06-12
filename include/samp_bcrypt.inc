
#if defined _inc_samp_bcrypt
	#undef _inc_samp_bcrypt
#endif

#if defined _samp_bcrypt_included
	#endinput
#endif
#define _samp_bcrypt_included

#if !defined BCRYPT_HASH_LENGTH
	#define BCRYPT_HASH_LENGTH (60 + 1)
#endif

#if !defined BCRYPT_COST
	#define BCRYPT_COST (12)
#endif

/*
bcrypt_hash(playerid, const callback[], const input[],cost,const args[] = "", {Float, _}:...)
	Params
		`playerid` - id of the player
		`callback[]` - callback to execute after hashing
		`input[]` - string to hash
		`cost` - work factor (4 - 31)
		`args` - custom arguements

	Example
		```
		main(){
			bcrypt_hash(0,"OnPassswordHash","text",12);
		}
		forward OnPassswordHash(playerid,hashid);
		public OnPassswordHash(playerid,hashid){
			//hashid is id of stored result in memory
		}
		```
*/
native bcrypt_hash(playerid, const callback[], const input[],cost,const args[] = "", {Float, _}:...);
/*
bcrypt_verify(playerid,callback[],input[],hash[])
	Params
		`playerid` - id of the player
		`callback[]` - callback to execute after hashing
		`input[]` - text to compare with hash
		`hash[]` - hash to compare with text
	Example
		```
		main(){
			bcrypt_verify(0,"OnPassswordVerify","text","$2y$12$lSzxFYNULh7weMGb8tf0beY1Lkb429nF.umuO/n0O.Q3U6wb1h5x.
");
		}
		forward OnPassswordVerify(playerid,bool:success);
		public OnPassswordVerify(playerid,bool:success){
			//success denotes verifying was successful or not
			if(success){
				//verfied
			} else{
				//hash doesn't match with text
			}
		}
		```
*/
native bcrypt_verify(playerid, const callback[], const input[], const hash[]);
/*
bcrypt_get_hash(dest[],size = sizeof(hash))
	Params
		`dest[]` - string to store hashed data
		`size` - max size of dest string
	Example
		```
		main(){
			bcrypt_hash(0,"OnPassswordHash","text",12);
		}
		forward OnPassswordHash(playerid,hashid);
		public OnPassswordHash(playerid,hashid){
			new dest[60];
			bcrypt_get_hash(dest);
			printf("hash : %s",dest);
		}
		```
*/
native bcrypt_get_hash(const hash[], size = sizeof(hash));

/*
bcrypt_set_thread_limit(value)
	Params
		`value` - number of worker threads at a time
	Example
		```
		main(){
			bcrypt_set_thread_limit(3);
		}
		```
*/
native bcrypt_set_thread_limit(value);
