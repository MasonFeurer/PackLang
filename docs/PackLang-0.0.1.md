# PackLang
The language documentation here is heavily in development.

## Including Other Datapack's
the `include` statement will include another datapack in the compilation of this one.
it can be given a relative or absolute path to the root of some other datapack

```
include "../other_pack";

function main() {
	other_pack:some_function();
}
```


## External Macros
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
	print!("you have {score:@p,tokens} tokens");
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





## Example 1.0
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
	at @a {
		if score_in_range(@p, get_boat, 1..) {
			reset(@p, get_boat);
			at the_stand {
				summon(boat);
			}
		}
	}
}

function get_boat_trigger() at player {
	reset(@p, get_boat);
	at the_stand {
		summon(boat);
	}
}




// in standard library
conditional score_in_range(p:target, o:objective, r:range) {
	unsafe {
		scoreboard players set $cond 0
		execute if score $p $o matches $r run scoreboard players set $cond 1
	}
}

command summon(e:entity_id) at anywhere {
	unsafe {
		// /summon is location-dependent
		summon $e
	}
}

command reset_all(o:objective) {
	reset(@a, $o);
}

command reset(p:target, o:objective) {
	unsafe {
		scoreboard players reset $p $o
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


# Functions

## At Modifier

`function`s can use an `at` modifier to specify that the function must be executed at some explicit location, with guards, restricting in what way must the location of the function call be known. Ex: a function defined with `at player` can only be called at a point where the command will be at a player, like if you used `at @a {}`.

Items that are in-lined (ex:command,conditional) do not use the `at` modifier, since they are not functions, instead a `target` is defined as an argument, and used in the commands.


## Main and Tick
the function called `main` is the function called on load.
if any imported libraries have a `main` function, those `main` functions must be called in your `main` function
^^^ similarly for `tick`


# Scoreboards

## Defining Scoreboards in Functions
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
command set(sc:score_name, o:objective, v:int) {
	unsafe {
		scoreboard players set $sc $o $v
	}
}
```



```
scoreboard dummy math;

function add(a, b -> result : math) {
	unsafe {
		scoreboard players operation result math = a math;
		scoreboard players operation result math += b math;
	}
}

command print_score(to:target, sc:score_name, obj:objective) {
	unsafe {
		tellraw $to ["score":{"name":"$sc","objective":"$obj"}];
	}
}

function main {
	// how to use return values?
	
	// this is bad code, because the scoreboard in which the return value is stored should not need to be known by the caller
	add(42, 83);
	print_score(@a, result, math);
	
	// maybe something like
	|sum = add(42, 83);
	print_score(@a, |sum);
	print_score(@a, |sum);
}

```

`main2` compiles to:
```
scoreboard players set a math 42
scoreboard players set b math 83
function add
tellraw @a ["score":{"name":"result","objective":"math"}]
tellraw @a ["score":{"name":"result","objective":"math"}]
```

```
scoreboard dummy math;

// takes 2 args: `a` and `b`
// returns 1 arg: `result`
// in the scoreboard `math`
function add(a, b -> result : math) {
	
}

function save_add(a, b : math) {
	
}

function get_sum(-> result : math) {
	
}

```



```
datapack stdlib;

function kill_player at player {
	unsafe { kill @p }
}
```


```

entity the_stand:armor_stand at [0, 0, 0] {
	Tags: ["the_stand"]
}

scoreboard dummy the_stand_pos;

function set_stand_x(x : the_stand_pos) {
	set_entity_x(!the_stand, x the_stand_pos);
}

command store_score_to_entity_data(
	t:target, path:nbt_path, ty:data_type, sc:score
) {
	unsafe {
		execute store result entity $t $path $ty 1 run scoreboard players get $sc
		// is it possible to exclude $ty?
	}
}

command set_entity_x(t:target, v:score) {
	unsafe {
		execute store result entity $t Pos[0] double 1 run scoreboard players get $v
	}
}

```
