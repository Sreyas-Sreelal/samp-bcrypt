# SampBcrypt
![Build](https://github.com/sreyas-sreelal/samp-bcrypt/actions/workflows/build.yml/badge.svg)
[![sampctl](https://img.shields.io/badge/sampctl-supported-2f2f2f.svg)](https://github.com/Sreyas-Sreelal/samp-bcrypt)
[![GitHub issues](https://img.shields.io/github/issues/Sreyas-Sreelal/samp-bcrypt.svg)](https://github.com/Sreyas-Sreelal/samp-bcrypt/issues) [![GitHub pull requests](https://img.shields.io/github/issues-pr-raw/sreyas-sreelal/samp-bcrypt.svg)](https://github.com/Sreyas-Sreelal/samp-bcrypt/pulls) [![GitHub pull license](https://img.shields.io/github/license/sreyas-sreelal/samp-bcrypt.svg)](LICENSE)

A bcrypt plugin for samp in Rust.

## Installation
### sampctl
If you are a sampctl user
`sampctl install Sreyas-Sreelal/samp-bcrypt`

### OR
* Download suitable binary files from releases for your operating system
* Add it your `plugins` folder
* Add `samp_bcrypt` to server.cfg or  `samp_bcrypt.so` (for linux)
* Add [samp_bcrypt.inc](include/samp_bcrypt.inc) in includes folder

## Building
* Clone the repo

	`git clone https://github.com/Sreyas-Sreelal/samp-bcrypt.git`
* Setup testing server

	`make setup`

* Build using makefile

	`make release` for release builds

	`make debug` for debug builds
* Run the tests

	`make run`

## API
* #### bcrypt_hash(playerid, const callback[], const input[], cost, const args[] = "", {Float, _}:...)
	* `playerid` - id of the player
	* `callback[]` - callback to execute after hashing
	* `input[]` - string to hash
	* `cost` - work factor (4 - 31)
	* `args[]` - custom arguments

	**Example**
	```Pawn
	main()
	{
		bcrypt_hash(0, "OnPassswordHash", "text", BCRYPT_COST);
	}

	forward OnPassswordHash(playerid);
	public OnPassswordHash(playerid)
	{
		// Hashing completed
	}
	```
* #### bcrypt_get_hash(dest[], size = sizeof(hash))
	* `dest[]` - string to store hashed data
	* `size` - max size of dest string

	**Example**
	```Pawn
	main()
	{
		bcrypt_hash(0, "OnPassswordHash", "text", BCRYPT_COST);
	}

	forward OnPassswordHash(playerid);
	public OnPassswordHash(playerid)
	{
		new dest[BCRYPT_HASH_LENGTH];
		bcrypt_get_hash(dest);
		printf("hash : %s", dest);
	}
	```
* #### bcrypt_verify(playerid, const callback[], const input[], const hash[], const args[] = "", {Float, _}:...)
	* `playerid` - id of the player
	* `callback[]` - callback to execute after hashing
	* `input[]` - text to compare with hash
	* `hash[]` - hash to compare with text
	* `args[]` - custom arguments
	
	**Example**
	```Pawn
	main()
	{
		bcrypt_hash(0, "OnPassswordHash", "text", BCRYPT_COST);
	}

	forward OnPassswordHash(playerid);
	public OnPassswordHash(playerid)
	{
		new dest[BCRYPT_HASH_LENGTH];
		bcrypt_get_hash(dest);
		bcrypt_verify(playerid, "OnPassswordVerify", "text", dest);
	}

	forward OnPassswordVerify(playerid, bool:success);
	public OnPassswordVerify(playerid, bool:success)
	{
		// success denotes verifying was successful or not
		
		if (success)
		{
			// Verified
		} 
		else
		{
			// Hash doesn't match with text
		}
	}
	```
* ### bcrypt_set_thread_limit(value)
	* `value` - number of worker threads at a time

	**Example**
	```Pawn
	main()
	{
		bcrypt_set_thread_limit(3);
	}
	```
