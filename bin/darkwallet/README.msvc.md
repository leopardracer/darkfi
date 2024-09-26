# Setting up the VM

You will need:

* Install QEMU in your distro. You should have the command `qemu-system-x86_64`.
* Install virt-viewer.
* Download the Windows 10 ISO from their website.
* Download virtio windows ISO

Then create a new image and a raw disk of 80 Gb.
See the [Arch QEMU guide](https://wiki.archlinux.org/title/QEMU).

Use this script to run the VM:

```
#!/bin/bash

ISO=virtio-win-0.1.262.iso
#ISO=virtio-win-0.1.217.iso
#ISO=Win10_22H2_EnglishInternational_x64v1.iso

args=(
    --cdrom $ISO --boot order=d

    -drive file=winblows-disk.raw,format=raw

    -m 30G -accel kvm -cpu qemu64

    # We forward 22 to 10022 for SSH
    #-net nic -net user,hostname=windowsvm
    -net nic -net user,hostname=windowsvm,hostfwd=tcp::10022-:22

    # This fixes the fucked up mouse
    #-device qemu-xhci -device usb-mouse -device usb-tablet
    -machine vmport=off

    # Auto-resize display
    # -vga qxl
    # We use virtio since it allows us the full res size at least
    -vga virtio -spice port=30001,disable-ticketing=on
    -device virtio-serial -chardev spicevmc,id=vdagent,debug=0,name=vdagent
    -device virtserialport,chardev=vdagent,name=com.redhat.spice.0
)

qemu-system-x86_64 "${args[@]}"
```

The connect to the VM with:

```
remote-viewer spice://localhost:30001
```

And install windows. Skip any stuff about the product key.

# Configure Windows

* Setup your resolution.
* Install spice guest additions from their website (scroll down).
* Install OpenSSH server: Settings -> Apps -> Optional features -> Add feature.
    * Select OpenSSH
    * Open Services, find OpenSSH, double click, enable autostart and click
      start.
    * Install Putty and try to connect over localhost.
* Install Visual Studio (not vscode) and select these workloads:
    * .NET desktop environment
    * Desktop development with C++
    * Windows application development
    * See [https://learn.microsoft.com/en-us/visualstudio/install/install-visual-studio?view=vs-2022](this guide).
* Install git for windows.
    * Make sure you select use Linux line endings instead of CRLF.
* Download & install rustup.
* Setup OpenGL using [this guide](https://thomas.inf3.ch/2019-06-12-opengl-kvm-mesa3d/index.html).
    * Download [mesa3d-xxx-release-msvc.7z](https://github.com/pal1000/mesa-dist-win/releases)
      and install the default options.

