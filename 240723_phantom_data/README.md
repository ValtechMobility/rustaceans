# Phantom Data
PhantomData is used to tell the compiler what to expect when handling structs. In our example you can only use specific function which are tied to a specific state. For example you can't use the `.authorize()` method on an already authorized `PasswordSafe`.

Regarding the overhead on "struct-creation" we don't have to worry about it. Those structs take up 0 space when compiled.