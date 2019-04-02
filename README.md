oreboot README
===============

oreboot is a downstream fork of coreboot, i.e. oreboot is coreboot without 'c'.

oreboot will only target truly open systems requiring no binary blobs.
oreboot is mostly written in Rust, with assembly where needed.

oreboot currently only plans to support LinuxBoot payloads.


Supported Hardware
------------------

coreboot supports a wide range of chipsets, devices, and mainboards, 
but no systems that require C or binary blobs. For now, that means no x86.

Build Requirements
------------------

 * Rust

Building oreboot
-----------------

Testing oreboot Without Modifying Your Hardware
------------------------------------------------

If you want to test oreboot without any risks before you really decide
to use it on your hardware, you can use the QEMU system emulator to run
oreboot virtually in QEMU.

Please see <https://www.oreboot.org/QEMU> for details.


Website and Mailing List
------------------------

Further details on the project, a FAQ, many HOWTOs, news, development
guidelines and more can be found on the oreboot website:

  <https://www.oreboot.org>

You can contact us directly on the oreboot mailing list:

  <https://www.oreboot.org/Mailinglist>


Copyright and License
---------------------

The copyright on oreboot is owned by quite a large number of individual
developers and companies. Please check the individual source files for details.

oreboot is licensed under the terms of the GNU General Public License (GPL).
Some files are licensed under the "GPL (version 2, or any later version)",
and some files are licensed under the "GPL, version 2". For some parts, which
were derived from other projects, other (GPL-compatible) licenses may apply.
Please check the individual source files for details.

This makes the resulting oreboot images licensed under the GPL, version 2.
