# SampBcrypt
A bcrypt plugin for samp in Rust.

## API 
* #### bcrypt_hash(playerid,callback[],input[],cost)
	* `playerid` - id of the player
	* `callback[]` - callback to execute after hashing
	* `input[]` - string to hash
	* `cost` - work factor (4 - 31)
	>usage
	```Pawn
	main(){
		bcrypt_hash(0,"OnPassswordHash","text",12);
	}

	forward OnPassswordHash(playerid,hashid);
	public OnPassswordHash(playerid,hashid){
		//hashid is id of stored result in memory
	}
	```
*  #### bcrypt_get_hash(context,dest[],size = sizeof(hash))
	* `context` - id of stored result
	* `dest[]` - string to store hashed data
	* `size` - max size of dest string
	>usage
	```Pawn
	main(){
		bcrypt_hash(0,"OnPassswordHash","text",12);
	}

	forward OnPassswordHash(playerid,hashid);
	public OnPassswordHash(playerid,hashid){
		new dest[250];
		bcrypt_get_hash(hashid,dest);
		printf("hash : %s",dest);
	}
	```
* #### bcrypt_verify(playerid,callback[],input[],hash[])
	* `playerid` - id of the player
	* `callback[]` - callback to execute after hashing
	* `input[]` - text to compare with hash
	* `hash[]` - hash to compare with text
	>usage
	```Pawn
	main(){
		bcrypt_hash(0,"OnPassswordHash","text",12);
	}

	forward OnPassswordHash(playerid,hashid);
	public OnPassswordHash(playerid,hashid){
		new dest[250];
		bcrypt_get_hash(hashid,dest);
		bcrypt_verify(playerid,"OnPassswordVerify","text",dest);
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
* #### bcrypt_delete(context)
	* `context` - id of stored result
	>usage
	```Pawn
	main(){
		bcrypt_hash(0,"OnPassswordHash","text",12);
	}

	forward OnPassswordHash(playerid,hashid);
	public OnPassswordHash(playerid,hashid){
		new dest[250];
		bcrypt_get_hash(hashid,dest);
		bcrypt_delete(hashid); //must be called after usage of dest is over
	}
	```