# AnsibleRoleTFTP

Ansible role for installing iPXE network booting firmwares.

## Example Playbook

An example playbook is provided in:
- [molecule/default/converge.tml](./molecule/default/converge.yml)

## Tests

To run the default test scenario locally:
- `molecule test -s default`

## Notes

- Network booting firmware should be present in the tftp serve directory, set with `tftp_directory`
in vars file, and the initial ipxe executable scripts should be present in the http serve directory, 
set with `http_directory` in the vars file.

- Edit the initial boot script before to your specifics before running.
