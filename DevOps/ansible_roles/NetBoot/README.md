# AnsibleRoleNetBoot

Ansible role for setting up a minimal netboot environment.

## Example Playbook

An example playbook is provided in:
- [molecule/default/converge.yml](./molecule/default/converge.yml)

## Tests

To run the default test scenario locally:
- `molecule test -s default`

## TODO
- support more distros, currently the installation server has some client distro agnostics abilities, 
but needs to be extended, especially for the case of grub and bios boot config files in templates directory,
also in finding the target kernels, which one option would be to get them as a variable from user.

- don't prompt for user input in bios menu, or in the default fedora kickstart file.

