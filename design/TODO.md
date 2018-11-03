## TODO

Make a globally accessible log function. It's nice that we've learned for ourselves why those are useful/necessary but now I get it.
    Make it a possibly not initialized `Logger` which is available from common.
        Reminder: we want `console.log` but we don't want to compile `std-web` every time. So assigning a function pointer from the precompiled crate seems like the simplest thing that could work.
        
        Let's use another feature like "invaraint_checking" to enable logging, so we don't need to even check during runtime in the release version and we can therefore leave the logs there

in in_game::Change selection screen if there are no changes made to the rules make the done button a cancel button.

When b button is pressed on the menus, jump to "cancel".
