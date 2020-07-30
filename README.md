# ZC - ZFS Commander
An ZFS administration tool inspired on Midnight commander

*Work in Progress*

## Description

ZFS Commander is a simple front-end for the most commonly used zpool & zfs commands. While the tool is a work-in-progress, 
several functions are already working. See the list of functions implemented in Usage section.

The tool is composed of 2 windows, each showing different ZFS related information. The tool will start with Datasets at left
and Snapshots at the right window. By pressing TAB, each window will cycle through the different modes: Pools, Datasets, 
Volumes and Snapshots.

![ZFS Commander](https://github.com/manoeldesouza/zc/blob/master/screenshots/zc-draft-01.png)
![ZFS Commander](https://github.com/manoeldesouza/zc/blob/master/screenshots/zc-draft-02.png)


## Installation 

ZFS commander is build on Rust language, so make sure to have Rust installed on your machine.
Instructions can be found here: https://www.rust-lang.org/tools/install 

To compile and install ZFS comamnder, run the following commands:

    $ sudo apt install build-essential ncurses-dev
    $ make
    $ sudo make install
    $ make clean

To uninstall run:

    $ sudo make uninstall


## Usage

As the execution of some ZFS operations require higher privilege run ZFS commander with sudo or doas:

    $ sudo zc


### Operation

Use TAB key to switch between the modes available. 

Use LEFT or RIGHT keys to navigate in between the two windows.

**Function Keys per Mode**

|  Key  |       Pool      |    Dataset    |    Snapshot   |     Volume    |
|:-----:|:---------------:|:-------------:|:-------------:|:-------------:|
|**F1** |         -       |        -      |        -      |        -      |
|**F2** |         -       |        -      |   *zfs diff*  |        -      |
|**F3** |         -       |        -      |        -      |        -      |
|**F4** |         -       |        -      |        -      |        -      |
|**F5** |         -       |*zfs snapshot* |  *zfs clone*  |*zfs snapshot* |
|**F6** |         -       |  *zfs rename* |  *zfs rename* |  *zfs rename* |
|**F7** |  *zpool scrub*  |  *zfs create* | *zfs rollback*|  *zfs create* |
|**F8** | *zpool destroy* | *zfs destroy* | *zfs destroy* | *zfs destroy* |
|**F9** |         -       |        -      |        -      |        -      |
|**F10**|       Exit      |      Exit     |      Exit     |      Exit     |
