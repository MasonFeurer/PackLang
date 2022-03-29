# PackLang
The language documentation here is in development.

## Functions
**Functions**:
- are sub-processes for executing code
- can be called by other functions
- are compiled to `.mcfunction` files

**Functions are defined as follows**:
1. `function <name>` : define the function with a name.
2. `()` : some parentheses, this is where the functions arguments will be.
3. optionally, `at <loc>` : define guards for where this function can be executed.
4. `{}` : the function itself, where all the code goes.

**Optionally, inside the parentheses**:
1. `<input>` : a comma-separated list of scores for the function to receive
2. optionally, `-> <output>` : a comma-separated list of scores the function will return
3. `: <objective>` : the objective where the input/outputs scores will be. Must be defined if either 1 or 2 are defined

**Examples**:
```
function main() {}

function sum(a, b -> sum : math) {}

function neg(x -> neg_x : math) {}

function print_nums(a, b : nums) {}

function get_count(-> count : some_board) {}

function kill_player() at player {}
```

## Compiling

## Running
The function named `main` will be called every reload.
The function named `tick` will be called every game-tick (20 tps)

## Scoreboards

### Scoreboards in Functions
A scoreboard defined in a function will be created in `load`, same as a scoreboard defined at the top-level.
Difference is that, a scoreboard defined in a function cannot be accessed by any other items.

A function-scoped scoreboard could be useful for temporary values a function may need to hold, but without exposing the scoreboard to other items.

### Example:
```
function main() {
	scoreboard dummy main_board;
	
	set(sc1, main_board, 4);
}

// in standard library
inline set(sc:score_name, o:objective, v:int) {
	unsafe {
		scoreboard players set $sc $o $v
	}
}
```

## Inlines

## External Macros
An external macro is a compiled binary, somewhere on your system, that will take a string (via the environment args), and return a new string (by printing it to standard output).

macro at `some_path/print-macro.exe`
```

external macro print = "some_path/print-macro.exe"

scoreboard dummy tokens;

function main() {
	at @a {
		print_bal();
	}
}

function print_bal at player {
	print!("you have {score:@p tokens} tokens");
	// calling the print macro expands this line to:
	tellraw @a ["you have ",{"score":{"name":"@p",objective:"tokens"}}," tokens"]
}
```


## Libraries
If any imported libraries have a `main` and/or `tick` function, those function must be called in your `main` function. <p> <p>

the `include` statement will include another datapack in the compilation of this one.
it can be given a relative or absolute path to the root of some other datapack.

**Example**:
```
include "../other_pack";

function main() {
	other_pack:some_function();
}
```


## Project Structure

items in `$PROJECT_DIR/datapack.mccs` will have the path `$DATAPACK:`

items in `$PROJECT_DIR/some_mod.mccs` will have the path `$DATAPACK:some_mod/`

items in `$PROJECT_DIR/some_mod/inner_mod.mccs` will have the path `$DATAPACK:some_mod/inner_mod/`

so every file in the project is a `module`

## Modules
Modules are basically folders in a file

In a Minecraft datapack, every function gets its own file.
But in MCCS, every function is defined in some file, representing the directory the file will end up in when compiled


So any function in the file `$PROJECT_DIR/some_mod.mccs`
will be placed in their own file in `/data/$DATAPACK/functions/some_mod/`




# END OF DOC
# The rest is just experimentation
```
scoreboard dummy math;

function add(a, b -> result : math) {
	unsafe {
		scoreboard players operation result math = a math;
		scoreboard players operation result math += b math;
	}
}

function main() {
	// how to use return values?
	
	// this is bad code because:
	//  - the scoreboard that stores the return should not need to be known by the caller
	//  - the scoreboard that stores the return may be function-scoped
	//  - it's clunky
	add(42, 83);
	print_score(@a, result, math);
	
	// maybe something like
	|sum = add(42, 83);
	print_score(@a, |sum);
	
	// creates a "pipe" by the name `sum`, which represents the return values of `add`
	// accessing `sum` with `|sum`, will fill in it's score name and objective
}

inline print_score(to:target, sc:score_name, obj:objective) {
	unsafe {
		tellraw $to [{"score":{"name":"$sc","objective":"$obj"}}];
	}
}
```

```
function kill_player() at player {
	unsafe { kill @p }
}
function main() {
	at @a { kill_player(); }
}

// vs

inline kill(p:target) {
	unsafe { kill $p }
}
function main() {
	kill(@a);
}
```

```
entity the_stand:armor_stand at [0, 0, 0] {
	Tags: ["the_stand"]
}

scoreboard dummy the_stand_pos;

function set_stand_x(x : the_stand_pos) {
	set_entity_x(!the_stand, x the_stand_pos);
	// `!the_stand` will expand to `@e[type=armor_stand,tag=the_stand,limit=1]`
	// `!the_stand` requires `the_stand` to have tags
}

inline set_entity_x(t:target, v:score) {
	// `v:score` is the same as `v1:target, v2:objective`
	unsafe {
		execute store result entity $t Pos[0] double 1 run scoreboard players get $v
	}
}

inline store_score_to_entity_data(
	t:target, path:nbt_path, ty:data_type, sc:score
) {
	unsafe {
		execute store result entity $t $path $ty 1 run scoreboard players get $sc
		// is it possible to exclude $ty?
	}
}
```


### Without MCCS
main
```
summon armor_stand 0 500 0 {NoGravity:1b,Invisible:1b,Marker:1b,Invincible:1b,Tags:["the_stand"]}
	
scoreboard objectives add get_boat trigger
```

tick
```
scoreboard players enable @a trigger
	
execute at @a if score @p get_boat matches 1.. run function $datapack:get_boat_trigger
```

get_boat_trigger
```
scoreboard players reset @p get_boat
	
execute at @e[tag=the_stand,limit=1] run summon boat
```

### With MCCS
```
entity armor_stand the_stand at [0, 500, 0] {
	NoGravity:1b,
	Invisible:1b,
	Marker:1b,
	Invincible:1b,
	Tags:["the_stand"]
}

scoreboard trigger get_boat {
	enabled: @a
}

function tick() {
	if score_in_range(@p get_boat, 1..) at @a {
		reset(@p get_boat);
		summon(boat) at !the_stand;
	}
}



// in standard library
cond score_in_range(sc:score, r:range) {
	unsafe {
		scoreboard players set $cond 0
		execute if score $sc matches $r run scoreboard players set $cond 1
	}
}

inline summon(e:entity_id) at any {
	unsafe {
		// /summon is location-dependent, therefor `at any`
		summon $e
	}
}

inline reset(sc:score) {
	unsafe {
		scoreboard players reset $sc
	}
}
```

### Compiled
load
```
scoreboard objectives add __tick dummy
scoreboard objectives add get_boat trigger
summon armor_stand 0 500 0 {NoGravity:1b,Invisible:1b,Invincible:1b,Tags:["the_stand"]}
```

tick
```
scoreboard players enable @a get_boat
execute at @a run function __tick1
```

\_\_tick1
```
scoreboard players set __cond1 __tick 0
execute at @p if score @p get_boat matches 1.. run score scoreboard players set __cond2 __tick 1
execute if score __cond1 __tick matches 1 run function __tick2
```

\_\_tick2
```
scoreboard players reset @p get_boat
execute at @e[tag=the_stand,limit=1] run summon boat
```



```
scoreboard dummy tokens;

function main() {
	at @a {
		print_bal();
	}
	// vs
	print_bal() at @a;
}

function print_bal at player {
	
}
```