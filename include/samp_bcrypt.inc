
#if defined _inc_samp_bcrypt
	#undef _inc_samp_bcrypt
#endif

#if defined _samp_bcrypt_included
	#endinput
#endif
#define _samp_bcrypt_included

native bcrypt_hash(playerid,callback[],input[],cost);
