# EstrOS
a microkernel/operating system written in rust

# Support
only supports Arm v8-a cpu's, and currently exclusively qemu but hardware is in the plans eventually

## developing
<sub>why would anyone ever wanna use this but me :thinking:</sub>

enter the dev environment
~~~sh
nix develop # if you dont use nix, you can find the package list in the flake
~~~
run with
~~~sh
krun            # release kernel + init
krun --debug    # debug kernel + init
~~~

debug with 
~~~sh
kdebug          # debug kernel + init, halted, with gdb attached
~~~

## credits
- based on [aarch64-bare-metal-qemu](https://github.com/freedomtan/aarch64-bare-metal-qemu/tree/master)
- adapted into rust by evelyn
- name by evelyn (different one from above)
- heavily insipired to do any of this in the first place by [developed from scratch](https://www.youtube.com/@DevelopedFromScratch)
