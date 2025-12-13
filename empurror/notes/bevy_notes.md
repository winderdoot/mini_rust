# Notes on bevy

Stuff i decided to note down because it's more niche

## Systems
Note that systems can be generic, you have to provide type via turbofish when you add them to the app

### Local
```Local<T>``` parameter in a system makes a stateful system, effectively a 
`static` variable owned by the function/system. It's initialized via FromWorld trait.
FromWorld is implemented for Default by well, default.

### SystemSet

Allows for grouping systems, that way we can make order dependencies on sets,
which makes the logic clearer.
- ```app.ad_systems()```
- ```system_function.in_set()```

### App States

States trait for implementing game state that determines which systems can run.

- ```system_function.run_if(in_state(MyStates::InGame))```

### Exclusive systems

Generally avoid them UNLESS you want to spawn/despawn many entities or do something that can only be done with commands.  
For example setting up the game world at the beginning/when new level is loaded
